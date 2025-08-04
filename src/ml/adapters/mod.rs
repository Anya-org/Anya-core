//! ML Model Adapters for External Framework Integration
//!
//! This module provides adapter patterns for integrating various ML frameworks
//! with the Anya Core ML system, following hexagonal architecture principles.

use crate::ml::real_inference::{InferenceRequest, InferenceResponse};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub mod burn_adapter;
pub mod candle_adapter;
pub mod huggingface_adapter;
pub mod ollama_adapter;
pub mod torch_adapter;

pub use burn_adapter::BurnAdapter;
pub use candle_adapter::CandleAdapter;
pub use huggingface_adapter::HuggingFaceAdapter;
pub use ollama_adapter::OllamaAdapter;
pub use torch_adapter::TorchAdapter;

/// Helper trait for type erasure (shared across adapters)
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Core ML adapter trait for plug-and-play model support
#[async_trait]
pub trait MLModelAdapter: Send + Sync {
    /// Load a model from the specified path/config
    async fn load_model(&self, config: ModelConfig) -> Result<Box<dyn LoadedModel>>;

    /// Run inference on a single input
    async fn inference(
        &self,
        model: &dyn LoadedModel,
        input: InferenceRequest,
    ) -> Result<InferenceResponse>;

    /// Run batch inference for better performance
    async fn batch_inference(
        &self,
        model: &dyn LoadedModel,
        inputs: Vec<InferenceRequest>,
    ) -> Result<Vec<InferenceResponse>>;

    /// Get supported model formats
    fn supported_formats(&self) -> Vec<ModelFormat>;

    /// Get hardware requirements for this adapter
    fn hardware_requirements(&self) -> HardwareRequirements;

    /// Get adapter name and version
    fn adapter_info(&self) -> AdapterInfo;
}

/// Model configuration for loading
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub model_path: String,
    pub model_type: ModelFormat,
    pub device_preference: DevicePreference,
    pub optimization_level: OptimizationLevel,
    pub max_batch_size: Option<usize>,
    pub precision: ModelPrecision,
    pub custom_config: HashMap<String, String>,
}

/// Supported model formats
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelFormat {
    ONNX,
    PyTorch,
    TensorFlow,
    Candle,
    Burn,
    HuggingFace,
    Safetensors,
    Custom(String),
}

/// Device preferences for model execution
#[derive(Debug, Clone)]
pub enum DevicePreference {
    CPU,
    GPU(u32), // GPU device ID
    Auto,
    Distributed(Vec<u32>), // Multiple devices
}

/// Model precision settings
#[derive(Debug, Clone)]
pub enum ModelPrecision {
    Float32,
    Float16,
    BFloat16,
    Int8,
    Auto,
}

/// Optimization levels
#[derive(Debug, Clone)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Custom(HashMap<String, String>),
}

/// Hardware requirements specification
#[derive(Debug, Clone)]
pub struct HardwareRequirements {
    pub min_memory_gb: f32,
    pub preferred_memory_gb: f32,
    pub requires_gpu: bool,
    pub min_gpu_memory_gb: Option<f32>,
    pub supported_architectures: Vec<String>,
}

/// Adapter information
#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub supported_features: Vec<String>,
}

/// Trait for loaded models
pub trait LoadedModel: Send + Sync + AsAny {
    fn model_id(&self) -> &str;
    fn model_format(&self) -> ModelFormat;
    fn memory_usage(&self) -> usize;
    fn supported_input_shapes(&self) -> Vec<Vec<usize>>;
}

/// Factory for creating adapters based on model format
pub struct AdapterFactory;

impl AdapterFactory {
    /// Create appropriate adapter for the given model format
    pub fn create_adapter(
        format: &ModelFormat,
        device: DevicePreference,
    ) -> Result<Box<dyn MLModelAdapter>> {
        match format {
            ModelFormat::Candle | ModelFormat::Safetensors | ModelFormat::HuggingFace => {
                let device = match device {
                    DevicePreference::GPU(device_id) => {
                        candle_adapter::CandleDevice::Cuda(device_id as usize)
                    }
                    DevicePreference::CPU => candle_adapter::CandleDevice::Cpu,
                    DevicePreference::Distributed(_) => candle_adapter::CandleDevice::Cpu, // Fallback to CPU
                    DevicePreference::Auto => {
                        if cfg!(feature = "cuda") {
                            candle_adapter::CandleDevice::Cuda(0)
                        } else {
                            candle_adapter::CandleDevice::Cpu
                        }
                    }
                };
                let adapter = CandleAdapter::new(device, candle_adapter::CandleConfig::default());
                Ok(Box::new(adapter))
            }
            ModelFormat::Custom(ref name) if name == "Ollama" => {
                let adapter = OllamaAdapter::new(ollama_adapter::OllamaConfig::default());
                Ok(Box::new(adapter))
            }
            ModelFormat::Burn => {
                let backend = match device {
                    DevicePreference::GPU(_) => burn_adapter::BurnBackend::Wgpu,
                    DevicePreference::CPU => burn_adapter::BurnBackend::NdArray,
                    DevicePreference::Distributed(_) => burn_adapter::BurnBackend::NdArray, // Fallback to CPU
                    DevicePreference::Auto => {
                        if cfg!(feature = "wgpu") {
                            burn_adapter::BurnBackend::Wgpu
                        } else {
                            burn_adapter::BurnBackend::NdArray
                        }
                    }
                };
                let adapter = BurnAdapter::new(backend, burn_adapter::BurnConfig::default());
                Ok(Box::new(adapter))
            }
            ModelFormat::PyTorch => {
                let torch_device = match device {
                    DevicePreference::GPU(device_id) => {
                        torch_adapter::TorchDevice::Cuda(device_id as i64)
                    }
                    DevicePreference::CPU => torch_adapter::TorchDevice::Cpu,
                    DevicePreference::Distributed(_) => torch_adapter::TorchDevice::Cpu, // Fallback to CPU
                    DevicePreference::Auto => torch_adapter::TorchAdapter::detect_device(),
                };
                let adapter =
                    TorchAdapter::new(torch_device, torch_adapter::TorchConfig::default());
                Ok(Box::new(adapter))
            }
            ModelFormat::ONNX | ModelFormat::TensorFlow => {
                // Fall back to existing RealMLEngine for these formats
                Err(anyhow::anyhow!(
                    "Format {:?} not supported by adapter factory, use RealMLEngine directly",
                    format
                ))
            }
            ModelFormat::Custom(_) => {
                Err(anyhow::anyhow!("Custom formats not supported by factory"))
            }
        }
    }

    /// Get recommended adapter for a model file
    pub fn recommend_adapter(model_path: &str) -> ModelFormat {
        let path_lower = model_path.to_lowercase();

        if path_lower.contains("safetensors") || path_lower.ends_with(".safetensors") {
            ModelFormat::Safetensors
        } else if path_lower.contains("candle") || path_lower.contains("hf") {
            ModelFormat::Candle
        } else if path_lower.contains("burn") || path_lower.ends_with(".burn") {
            ModelFormat::Burn
        } else if path_lower.contains("pytorch")
            || path_lower.ends_with(".pt")
            || path_lower.ends_with(".pth")
        {
            ModelFormat::PyTorch
        } else if path_lower.ends_with(".onnx") {
            ModelFormat::ONNX
        } else if path_lower.contains("tensorflow") || path_lower.ends_with(".pb") {
            ModelFormat::TensorFlow
        } else {
            // Default to Candle for Hugging Face models
            ModelFormat::Candle
        }
    }
}

/// Registry for managing multiple adapters
pub struct MLAdapterRegistry {
    adapters: HashMap<ModelFormat, Arc<dyn MLModelAdapter>>,
    default_adapter: Option<Arc<dyn MLModelAdapter>>,
    auto_selection_enabled: bool,
}

impl MLAdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            default_adapter: None,
            auto_selection_enabled: true,
        }
    }

    /// Register a new adapter for a specific model format
    pub fn register_adapter<T>(&mut self, format: ModelFormat, adapter: T)
    where
        T: MLModelAdapter + 'static,
    {
        self.adapters.insert(format, Arc::new(adapter));
    }

    /// Set the default adapter for unknown formats
    pub fn set_default_adapter<T>(&mut self, adapter: T)
    where
        T: MLModelAdapter + 'static,
    {
        self.default_adapter = Some(Arc::new(adapter));
    }

    /// Get adapter for specific format
    pub fn get_adapter(&self, format: &ModelFormat) -> Option<Arc<dyn MLModelAdapter>> {
        self.adapters
            .get(format)
            .cloned()
            .or_else(|| self.default_adapter.clone())
    }

    /// Auto-select best adapter for a model file
    pub async fn auto_select_adapter(&self, model_path: &str) -> Result<Arc<dyn MLModelAdapter>> {
        let format = self.detect_model_format(model_path).await?;
        self.get_adapter(&format)
            .ok_or_else(|| anyhow::anyhow!("No adapter available for format: {:?}", format))
    }

    /// Detect model format from file
    async fn detect_model_format(&self, model_path: &str) -> Result<ModelFormat> {
        // Implementation to detect model format from file extension, headers, etc.
        if model_path.ends_with(".onnx") {
            Ok(ModelFormat::ONNX)
        } else if model_path.ends_with(".pt") || model_path.ends_with(".pth") {
            Ok(ModelFormat::PyTorch)
        } else if model_path.ends_with(".pb") {
            Ok(ModelFormat::TensorFlow)
        } else if model_path.ends_with(".safetensors") {
            Ok(ModelFormat::Safetensors)
        } else {
            // Try to detect from file contents
            Ok(ModelFormat::Custom("unknown".to_string()))
        }
    }

    /// List all available adapters
    pub fn list_adapters(&self) -> Vec<(ModelFormat, AdapterInfo)> {
        self.adapters
            .iter()
            .map(|(format, adapter)| (format.clone(), adapter.adapter_info()))
            .collect()
    }
}

impl Default for MLAdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_registry() {
        let registry = MLAdapterRegistry::new();

        // Test would register mock adapters and verify functionality
        assert_eq!(registry.list_adapters().len(), 0);
    }
}
