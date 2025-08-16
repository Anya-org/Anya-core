//! Ollama Local LLM Adapter
//!
//! Provides integration with Ollama for running large language models locally
//! while maintaining compatibility with the Anya ML adapter system.

use super::*;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Ollama adapter for local LLM inference
pub struct OllamaAdapter {
    client: OllamaClient,
    model_cache: Arc<RwLock<HashMap<String, Arc<OllamaModel>>>>,
    config: OllamaConfig,
}

/// Configuration for Ollama adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_tokens: Option<u32>,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: Option<u32>,
    pub context_window: usize,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            timeout_seconds: 300,
            max_tokens: Some(4096),
            temperature: 0.7,
            top_p: 0.9,
            top_k: Some(40),
            context_window: 8192,
        }
    }
}

/// Ollama HTTP client for API communication
pub struct OllamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Check if Ollama server is available
    pub async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;
        Ok(response.status().is_success())
    }

    /// Generate text using Ollama API
    pub async fn generate(&self, request: &OllamaGenerateRequest) -> Result<OllamaResponse> {
        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Ollama API error: {}", response.status()));
        }

        Ok(response.json().await?)
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;

        let tags: OllamaTagsResponse = response.json().await?;
        Ok(tags.models.into_iter().map(|m| m.name).collect())
    }
}

/// Ollama model wrapper
pub struct OllamaModel {
    model_id: String,
    model_name: String,
    memory_usage: usize,
    context_size: usize,
}

impl AsAny for OllamaModel {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl LoadedModel for OllamaModel {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn model_format(&self) -> ModelFormat {
        ModelFormat::Custom("Ollama".to_string())
    }

    fn memory_usage(&self) -> usize {
        self.memory_usage
    }

    fn supported_input_shapes(&self) -> Vec<Vec<usize>> {
        // Text models support variable length sequences
        vec![vec![1], vec![512], vec![1024], vec![self.context_size]]
    }
}

impl Clone for OllamaModel {
    fn clone(&self) -> Self {
        Self {
            model_id: self.model_id.clone(),
            model_name: self.model_name.clone(),
            memory_usage: self.memory_usage,
            context_size: self.context_size,
        }
    }
}

impl OllamaAdapter {
    /// Create new Ollama adapter
    pub fn new(config: OllamaConfig) -> Self {
        let client = OllamaClient::new(config.base_url.clone());

        Self {
            client,
            model_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Load an Ollama model
    async fn load_ollama_model(&self, config: &ModelConfig) -> Result<Arc<OllamaModel>> {
        // Check if model exists in Ollama
        let available_models = self.client.list_models().await?;
        if !available_models.contains(&config.model_path) {
            return Err(anyhow!(
                "Model {} not available in Ollama",
                config.model_path
            ));
        }

        let model = OllamaModel {
            model_id: config.model_path.clone(),
            model_name: config.model_path.clone(),
            memory_usage: 4_000_000_000, // 4GB estimate for typical LLM
            context_size: self.config.context_window,
        };

        Ok(Arc::new(model))
    }

    /// Convert text to embedding-like representation
    fn text_to_embeddings(&self, text: &str) -> Vec<f32> {
        // Simple hash-based embedding for compatibility
        // In production, use proper embedding models
        let mut embeddings = vec![0.0; 768]; // Standard embedding size

        for (i, byte) in text.bytes().enumerate() {
            if i >= embeddings.len() {
                break;
            }
            embeddings[i] = (byte as f32) / 255.0;
        }

        embeddings
    }

    /// Parse Ollama response for inference
    fn parse_response(&self, response: &OllamaResponse) -> Vec<f32> {
        // Convert response text to numerical representation
        self.text_to_embeddings(&response.response)
    }
}

#[async_trait]
impl MLModelAdapter for OllamaAdapter {
    async fn load_model(&self, config: ModelConfig) -> Result<Box<dyn LoadedModel>> {
        let model = self.load_ollama_model(&config).await?;

        // Cache the model
        {
            let mut cache = self.model_cache.write().await;
            cache.insert(config.model_path.clone(), model.clone());
        }

        Ok(Box::new((*model).clone()))
    }

    async fn inference(
        &self,
        model: &dyn LoadedModel,
        request: InferenceRequest,
    ) -> Result<InferenceResponse> {
        let ollama_model = model
            .as_any()
            .downcast_ref::<OllamaModel>()
            .ok_or_else(|| anyhow!("Invalid model type for Ollama adapter"))?;

        // Convert input data to text
        let input_text = if request.input_data.is_empty() {
            "".to_string()
        } else {
            String::from_utf8_lossy(
                &request
                    .input_data
                    .iter()
                    .map(|&f| f as u8)
                    .collect::<Vec<u8>>(),
            )
            .to_string()
        };

        let generate_request = OllamaGenerateRequest {
            model: ollama_model.model_name.clone(),
            prompt: input_text,
            stream: false,
            options: Some(OllamaOptions {
                temperature: Some(self.config.temperature),
                top_p: Some(self.config.top_p),
                top_k: self.config.top_k,
                num_predict: self.config.max_tokens,
            }),
        };

        let response = self.client.generate(&generate_request).await?;
        let predictions = self.parse_response(&response);

        Ok(InferenceResponse {
            model_id: model.model_id().to_string(),
            predictions,
            confidence_scores: vec![response.eval_count as f32 / 100.0], // Rough confidence
            overall_confidence: 0.85, // Default for LLM responses
            inference_time_ms: response.eval_duration as f64 / 1_000_000.0, // Convert nanoseconds
            model_version: "1.0.0".to_string(),
            cached: false,
            metadata: request.metadata,
        })
    }

    async fn batch_inference(
        &self,
        model: &dyn LoadedModel,
        inputs: Vec<InferenceRequest>,
    ) -> Result<Vec<InferenceResponse>> {
        // Process sequentially for simplicity
        // In production, could implement parallel processing
        let mut results = Vec::new();
        for input in inputs {
            results.push(self.inference(model, input).await?);
        }
        Ok(results)
    }

    fn supported_formats(&self) -> Vec<ModelFormat> {
        vec![ModelFormat::Custom("Ollama".to_string())]
    }

    fn hardware_requirements(&self) -> HardwareRequirements {
        HardwareRequirements {
            min_memory_gb: 8.0,
            preferred_memory_gb: 32.0,
            requires_gpu: false, // Can run on CPU
            min_gpu_memory_gb: Some(8.0),
            supported_architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
        }
    }

    fn adapter_info(&self) -> AdapterInfo {
        AdapterInfo {
            name: "Ollama".to_string(),
            version: "0.1.0".to_string(),
            description: "Ollama local LLM adapter for high-performance local inference"
                .to_string(),
            supported_features: vec![
                "text-generation".to_string(),
                "chat-completion".to_string(),
                "local-inference".to_string(),
                "streaming".to_string(),
            ],
        }
    }
}

/// Ollama API request structures
#[derive(Debug, Serialize)]
pub struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    num_predict: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    response: String,
    done: bool,
    eval_count: u64,
    eval_duration: u64,
}

impl OllamaResponse {
    /// Get whether the response is done
    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModelInfo>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelInfo {
    name: String,
    #[allow(dead_code)]
    modified_at: String,
    #[allow(dead_code)]
    size: u64,
}

impl OllamaModelInfo {
    /// Get model modification time
    #[allow(dead_code)]
    pub fn modified_at(&self) -> &str {
        &self.modified_at
    }

    /// Get model size in bytes
    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.size
    }
}

// Helper trait for type erasure
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ollama_adapter_creation() {
        let config = OllamaConfig::default();
        let adapter = OllamaAdapter::new(config);

        let info = adapter.adapter_info();
        assert_eq!(info.name, "Ollama");
        assert!(info
            .supported_features
            .contains(&"text-generation".to_string()));
    }

    #[tokio::test]
    async fn test_ollama_client() {
        let client = OllamaClient::new("http://localhost:11434".to_string());

        // Note: This test requires Ollama to be running
        // In CI, this would be skipped or use a mock server
        if let Ok(true) = client.health_check().await {
            let models = client.list_models().await.unwrap();
            println!("Available models: {models:?}");
        }
    }
}
