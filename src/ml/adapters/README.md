# ml/adapters Module

ML Model Adapters for External Framework Integration

This module provides adapter patterns for integrating various ML frameworks
with the Anya Core ML system, following hexagonal architecture principles.

## Overview

The `ml/adapters` module implements a flexible adapter framework that allows the Anya Core system to seamlessly integrate with multiple ML frameworks and model formats. It follows the adapter pattern from hexagonal architecture, creating a clean separation between the core ML logic and specific framework implementations.

## Key Components

### MLModelAdapter Trait

The central interface that all adapters must implement:

- `load_model`: Load models from paths/configurations
- `inference`: Run predictions on a single input
- `batch_inference`: Process multiple inputs efficiently
- `supported_formats`: List compatible model formats
- `hardware_requirements`: Specify necessary compute resources
- `adapter_info`: Provide metadata about the adapter

### Supported Adapters

The module includes adapters for popular ML frameworks:

- **Candle Adapter**: Native Rust ML framework from Hugging Face
- **Torch Adapter**: Interface for PyTorch models
- **Burn Adapter**: Native Rust deep learning framework
- **Ollama Adapter**: Integration with Ollama local models
- **HuggingFace Adapter**: Direct access to Hugging Face models

### Helper Components

- **AdapterFactory**: Creates appropriate adapters based on model format
- **MLAdapterRegistry**: Maintains and manages multiple adapters
- **ModelConfig**: Unified configuration for all frameworks
- **LoadedModel**: Common interface for loaded ML models

## Usage Examples

```rust
// Create an ML adapter registry
let mut registry = MLAdapterRegistry::new();

// Register different adapters
let candle_adapter = CandleAdapter::new(CandleDevice::Auto, CandleConfig::default());
registry.register_adapter(ModelFormat::Candle, candle_adapter);

// Configure model parameters
let config = ModelConfig {
    model_path: "llama2-7b-chat",
    model_type: ModelFormat::HuggingFace,
    device_preference: DevicePreference::GPU(0),
    optimization_level: OptimizationLevel::Basic,
    precision: ModelPrecision::Float16,
    max_batch_size: Some(8),
    custom_config: HashMap::new(),
};

// Load and use a model
let adapter = registry.get_adapter(&ModelFormat::HuggingFace).unwrap();
let model = adapter.load_model(config).await?;

// Perform inference
let result = adapter.inference(&*model, request).await?;
println!("Prediction: {:?}", result.predictions);
```

## Framework Integration

This module supports multiple model formats and frameworks:

| Framework/Format | Adapter              | Hardware Support    | Use Cases                         |
|------------------|----------------------|--------------------|-----------------------------------|
| Candle           | `CandleAdapter`      | CPU, CUDA, Metal   | LLMs, embeddings, transformers    |
| PyTorch          | `TorchAdapter`       | CPU, CUDA          | General ML models                 |
| Burn             | `BurnAdapter`        | CPU, WGPU          | Neural networks, native Rust      |
| HuggingFace      | `HuggingFaceAdapter` | CPU, CUDA          | Pre-trained models, transformers  |
| ONNX             | via `RealMLEngine`   | CPU, CUDA, various | Cross-framework models            |

## For more information

See the comprehensive documentation in the [docs/](../../../docs/) directory.
