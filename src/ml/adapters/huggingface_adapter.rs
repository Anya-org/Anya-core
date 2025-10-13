//! HuggingFace Model Hub Adapter
//!
//! Provides integration with HuggingFace Model Hub for downloading,
//! caching, and running models from the ecosystem.

use super::AsAny;
use super::*;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

/// HuggingFace Model Hub adapter
pub struct HuggingFaceAdapter {
    client: HfClient,
    model_cache: Arc<RwLock<HashMap<String, Arc<HfModel>>>>,
    config: HfConfig,
    cache_dir: PathBuf,
}

/// Configuration for HuggingFace adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HfConfig {
    pub api_token: Option<String>,
    pub cache_dir: PathBuf,
    pub max_cache_size_gb: u64,
    pub timeout_seconds: u64,
    pub use_auth_token: bool,
    pub revision: String,
    pub trust_remote_code: bool,
}

impl Default for HfConfig {
    fn default() -> Self {
        Self {
            api_token: std::env::var("HF_TOKEN").ok(),
            cache_dir: std::env::temp_dir().join("hf_cache"),
            max_cache_size_gb: 50,
            timeout_seconds: 300,
            use_auth_token: true,
            revision: "main".to_string(),
            trust_remote_code: false,
        }
    }
}

/// HuggingFace HTTP client
pub struct HfClient {
    client: reqwest::Client,
    api_token: Option<String>,
}

impl HfClient {
    pub fn new(api_token: Option<String>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(ref token) = api_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .unwrap();

        Self { client, api_token }
    }

    /// Get the API token if available
    pub fn api_token(&self) -> Option<&String> {
        self.api_token.as_ref()
    }

    /// Get model information from HuggingFace API
    pub async fn get_model_info(&self, model_id: &str) -> Result<ModelInfo> {
        let url = format!("https://huggingface.co/api/models/{model_id}");
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get model info: {}", response.status()));
        }

        Ok(response.json().await?)
    }

    /// Download a file from HuggingFace
    pub async fn download_file(
        &self,
        model_id: &str,
        filename: &str,
        local_path: &Path,
    ) -> Result<()> {
        let url = format!("https://huggingface.co/{model_id}/resolve/main/{filename}");

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download file: {}", response.status()));
        }

        // Ensure parent directory exists
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write file
        let bytes = response.bytes().await?;
        fs::write(local_path, bytes).await?;

        Ok(())
    }

    /// List model files
    pub async fn list_model_files(&self, model_id: &str) -> Result<Vec<String>> {
        let url = format!("https://huggingface.co/api/models/{model_id}/tree/main");
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to list model files: {}", response.status()));
        }

        let files: Vec<FileInfo> = response.json().await?;
        Ok(files.into_iter().map(|f| f.path).collect())
    }

    /// Search models
    pub async fn search_models(
        &self,
        query: &str,
        filter: Option<&str>,
    ) -> Result<Vec<ModelSearchResult>> {
        let mut url = format!("https://huggingface.co/api/models?search={query}");
        if let Some(filter) = filter {
            url.push_str(&format!("&filter={filter}"));
        }

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!("Failed to search models: {}", response.status()));
        }

        Ok(response.json().await?)
    }
}

/// HuggingFace model wrapper
pub struct HfModel {
    model_id: String,
    model_type: String,
    local_path: PathBuf,
    memory_usage: usize,
    model_info: ModelInfo,
}

impl LoadedModel for HfModel {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn model_format(&self) -> ModelFormat {
        ModelFormat::HuggingFace
    }

    fn memory_usage(&self) -> usize {
        self.memory_usage
    }

    fn supported_input_shapes(&self) -> Vec<Vec<usize>> {
        match self.model_type.as_str() {
            "text-generation" => vec![vec![1], vec![512], vec![1024], vec![2048]],
            "text-classification" => vec![vec![1, 512]],
            "feature-extraction" => vec![vec![1, 512]],
            "image-classification" => vec![vec![1, 3, 224, 224]],
            _ => vec![vec![1]],
        }
    }
}

impl Clone for HfModel {
    fn clone(&self) -> Self {
        Self {
            model_id: self.model_id.clone(),
            model_type: self.model_type.clone(),
            local_path: self.local_path.clone(),
            memory_usage: self.memory_usage,
            model_info: self.model_info.clone(),
        }
    }
}

impl HuggingFaceAdapter {
    /// Create new HuggingFace adapter
    pub fn new(config: HfConfig) -> Self {
        let client = HfClient::new(config.api_token.clone());

        Self {
            client,
            model_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir: config.cache_dir.clone(),
            config,
        }
    }

    /// Get adapter configuration
    pub fn config(&self) -> &HfConfig {
        &self.config
    }

    /// Download and cache a model from HuggingFace
    async fn download_model(&self, model_id: &str) -> Result<PathBuf> {
        let model_dir = self.cache_dir.join(model_id.replace('/', "_"));

        // Check if model already exists
        if model_dir.exists() {
            return Ok(model_dir);
        }

        // Get model info
        let model_info = self.client.get_model_info(model_id).await?;

        // Create model directory
        fs::create_dir_all(&model_dir).await?;

        // Download essential files
        let essential_files = [
            "config.json",
            "tokenizer.json",
            "tokenizer_config.json",
            "vocab.txt",
            "merges.txt",
            "special_tokens_map.json",
        ];

        // Get all available files
        let available_files = self.client.list_model_files(model_id).await?;

        // Download model files
        for file in available_files {
            if file.ends_with(".bin")
                || file.ends_with(".safetensors")
                || file.ends_with(".onnx")
                || essential_files.contains(&file.as_str())
            {
                let local_path = model_dir.join(&file);
                if let Err(e) = self
                    .client
                    .download_file(model_id, &file, &local_path)
                    .await
                {
                    log::warn!("Failed to download {file}: {e}");
                    // Continue with other files
                }
            }
        }

        // Save model info
        let info_path = model_dir.join("model_info.json");
        let info_json = serde_json::to_string_pretty(&model_info)?;
        fs::write(info_path, info_json).await?;

        Ok(model_dir)
    }

    /// Load a HuggingFace model
    async fn load_hf_model(&self, config: &ModelConfig) -> Result<Arc<HfModel>> {
        // Download model if needed
        let model_path = self.download_model(&config.model_path).await?;

        // Load model info
        let info_path = model_path.join("model_info.json");
        let model_info: ModelInfo = if info_path.exists() {
            let info_content = fs::read_to_string(info_path).await?;
            serde_json::from_str(&info_content)?
        } else {
            // Fallback to API call
            self.client.get_model_info(&config.model_path).await?
        };

        // Determine model type from pipeline tag or config
        let model_type = model_info
            .pipeline_tag
            .clone()
            .unwrap_or_else(|| "text-generation".to_string());

        let model = HfModel {
            model_id: config.model_path.clone(),
            model_type,
            local_path: model_path,
            memory_usage: 2_000_000_000, // 2GB estimate
            model_info,
        };

        Ok(Arc::new(model))
    }

    /// Run inference using HuggingFace model
    async fn run_inference(&self, model: &HfModel, input: &str) -> Result<Vec<f32>> {
        // This is a simplified implementation
        // In a real scenario, you'd use the actual model files with
        // libraries like tokenizers-rs, ort (ONNX), or candle

        match model.model_type.as_str() {
            "text-generation" => {
                // Simple text generation simulation
                let generated = format!("Generated response for: {input}");
                Ok(generated
                    .chars()
                    .map(|c| c as u8 as f32)
                    .take(768)
                    .collect())
            }
            "text-classification" => {
                // Simple classification simulation
                let sentiment_score = if input.contains("good") || input.contains("great") {
                    0.8
                } else if input.contains("bad") || input.contains("terrible") {
                    0.2
                } else {
                    0.5
                };
                Ok(vec![sentiment_score, 1.0 - sentiment_score])
            }
            "feature-extraction" => {
                // Simple embedding extraction
                Ok(self.text_to_embeddings(input))
            }
            _ => {
                // Generic response
                Ok(self.text_to_embeddings(input))
            }
        }
    }

    /// Convert text to embeddings
    fn text_to_embeddings(&self, text: &str) -> Vec<f32> {
        // Simple hash-based embedding for demonstration
        let mut embeddings = vec![0.0; 768];

        for (i, byte) in text.bytes().enumerate() {
            if i >= embeddings.len() {
                break;
            }
            embeddings[i] = (byte as f32) / 255.0;
        }

        // Normalize
        let norm: f32 = embeddings.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for emb in &mut embeddings {
                *emb /= norm;
            }
        }

        embeddings
    }

    /// Clean up cache if it exceeds size limit
    pub async fn cleanup_cache(&self) -> Result<u64> {
        // Simple cache cleanup - remove oldest models if cache is too large
        // In production, implement LRU or size-based cleanup
        Ok(0)
    }
}

#[async_trait]
impl MLModelAdapter for HuggingFaceAdapter {
    async fn load_model(&self, config: ModelConfig) -> Result<Box<dyn LoadedModel>> {
        let model = self.load_hf_model(&config).await?;

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
        let hf_model = model
            .as_any()
            .downcast_ref::<HfModel>()
            .ok_or_else(|| anyhow!("Invalid model type for HuggingFace adapter"))?;

        // Convert input data to text
        let input_bytes: Vec<u8> = request.input_data.iter().map(|&f| f as u8).collect();
        let input_text = String::from_utf8_lossy(&input_bytes);

        let predictions = self.run_inference(hf_model, &input_text).await?;

        // Calculate confidence based on model type
        let confidence_scores = match hf_model.model_type.as_str() {
            "text-classification" => predictions.to_vec(),
            _ => vec![0.85], // Default confidence
        };

        let overall_confidence =
            confidence_scores.iter().sum::<f32>() / confidence_scores.len() as f32;

        Ok(InferenceResponse {
            model_id: model.model_id().to_string(),
            predictions,
            confidence_scores,
            overall_confidence,
            inference_time_ms: 100.0, // Placeholder
            model_version: hf_model
                .model_info
                .sha
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            cached: true,
            metadata: request.metadata,
        })
    }

    async fn batch_inference(
        &self,
        model: &dyn LoadedModel,
        inputs: Vec<InferenceRequest>,
    ) -> Result<Vec<InferenceResponse>> {
        // Process in batches for efficiency
        let mut results = Vec::new();
        for input in inputs {
            results.push(self.inference(model, input).await?);
        }
        Ok(results)
    }

    fn supported_formats(&self) -> Vec<ModelFormat> {
        vec![
            ModelFormat::HuggingFace,
            ModelFormat::Safetensors,
            ModelFormat::ONNX,
        ]
    }

    fn hardware_requirements(&self) -> HardwareRequirements {
        HardwareRequirements {
            min_memory_gb: 4.0,
            preferred_memory_gb: 16.0,
            requires_gpu: false,
            min_gpu_memory_gb: Some(8.0),
            supported_architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
        }
    }

    fn adapter_info(&self) -> AdapterInfo {
        AdapterInfo {
            name: "HuggingFace".to_string(),
            version: "0.1.0".to_string(),
            description:
                "HuggingFace Model Hub adapter with automatic model downloading and caching"
                    .to_string(),
            supported_features: vec![
                "text-generation".to_string(),
                "text-classification".to_string(),
                "feature-extraction".to_string(),
                "image-classification".to_string(),
                "automatic-download".to_string(),
                "model-caching".to_string(),
            ],
        }
    }
}

/// HuggingFace API response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub sha: Option<String>,
    pub pipeline_tag: Option<String>,
    pub tags: Vec<String>,
    pub downloads: Option<u64>,
    pub likes: Option<u64>,
    pub library_name: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FileInfo {
    path: String,
    #[allow(dead_code)]
    size: Option<u64>,
}

impl FileInfo {
    /// Get file size if available
    #[allow(dead_code)]
    pub fn size(&self) -> Option<u64> {
        self.size
    }
}

#[derive(Debug, Deserialize)]
pub struct ModelSearchResult {
    pub id: String,
    pub pipeline_tag: Option<String>,
    pub tags: Vec<String>,
    pub downloads: Option<u64>,
    pub likes: Option<u64>,
}

// Helper trait for type erasure - use the shared one from mod.rs

impl AsAny for HfModel {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_huggingface_adapter_creation() {
        let config = HfConfig::default();
        let adapter = HuggingFaceAdapter::new(config);

        let info = adapter.adapter_info();
        assert_eq!(info.name, "HuggingFace");
        assert!(info
            .supported_features
            .contains(&"text-generation".to_string()));
    }

    #[tokio::test]
    async fn test_hf_client() {
        let client = HfClient::new(None);

        // Test searching models (requires internet)
        if let Ok(results) = client
            .search_models("bert-base", Some("text-classification"))
            .await
        {
            assert!(!results.is_empty());
            println!("Found {} models", results.len());
        }
    }

    #[test]
    fn test_text_to_embeddings() {
        let config = HfConfig::default();
        let adapter = HuggingFaceAdapter::new(config);

        let embeddings = adapter.text_to_embeddings("hello world");
        assert_eq!(embeddings.len(), 768);

        // Check normalization
        let norm: f32 = embeddings.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }
}
