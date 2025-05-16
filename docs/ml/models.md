# Model Management

This document outlines the model management system in Anya Core's ML infrastructure.

## Model Registry

### Registering a New Model

```python
from anya_ml import ModelRegistry

registry = ModelRegistry()
model_id = registry.register_model(
    name="sentiment-analysis",
    version="1.0.0",
    framework="pytorch",
    path="/path/to/model.pth",
    metrics={"accuracy": 0.95, "f1": 0.92}
)
```

### Model Versioning

Models follow semantic versioning (MAJOR.MINOR.PATCH):
- MAJOR: Breaking changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes and patches

## Model Serving

### Starting a Model Server

```bash
anya-ml serve --model-id sentiment-analysis:1.0.0 --port 8080
```

### Making Predictions

```python
import requests

response = requests.post(
    "http://localhost:8080/predict",
    json={"text": "Anya Core is amazing!"}
)
print(response.json())
```

## Model Monitoring

### Metrics Collection

Key metrics are automatically collected:
- Prediction latency
- Throughput
- Error rates
- Resource usage

### Alerting

Configure alerts for:
- High prediction latency
- Increased error rates
- Model drift
- Resource constraints

## Model Updates

### Rolling Updates

```bash
# Start canary deployment
anya-ml update --model-id sentiment-analysis:2.0.0 --strategy canary --percentage 10

# Monitor canary performance
anya-ml monitor --model-id sentiment-analysis:2.0.0

# Complete rollout
anya-ml update --model-id sentiment-analysis:2.0.0 --strategy rolling --batch-size 20%
```

### Rollback Procedure

```bash
# Check rollback targets
anya-ml history --model-id sentiment-analysis

# Rollback to previous version
anya-ml rollback --model-id sentiment-analysis --to-version 1.0.0
```

## Model Security

### Access Control

```yaml
# .anya/model_permissions.yaml
models:
  sentiment-analysis:
    read:
      - team:ml
    write:
      - user:admin
    admin:
      - user:ml-admin
```

### Model Signing

All models are cryptographically signed:
```bash
# Sign a model
anya-ml sign --model-id sentiment-analysis:1.0.0 --key ~/.keys/private.pem

# Verify model signature
anya-ml verify --model-id sentiment-analysis:1.0.0 --key ~/.keys/public.pem
```

## Best Practices

### Model Packaging

1. Include all dependencies in `requirements.txt`
2. Provide example input/output in `examples/`
3. Document model architecture in `README.md`
4. Include evaluation metrics and test results

### Performance Optimization

- Use ONNX for cross-framework optimization
- Enable model quantization for inference
- Utilize hardware acceleration (CUDA, MPS, etc.)
- Implement request batching

## Troubleshooting

### Common Issues

1. **Model Loading Failures**
   ```bash
   # Check model format
   file /path/to/model
   
   # Verify dependencies
   pip freeze | grep -E 'torch|tensorflow|onnx'
   ```

2. **Performance Issues**
   ```bash
   # Profile model
   anya-ml profile --model-id sentiment-analysis:1.0.0 --input /path/to/test_data.json
   ```

3. **Permission Errors**
   ```bash
   # Check model permissions
   ls -l /path/to/model
   
   # Verify API keys
   anya-ml config list
   ```

## Getting Help

- [API Reference](https://docs.anya.org/ml/api)
- [Model Development Guide](https://docs.anya.org/ml/development)
- [Troubleshooting Guide](https://docs.anya.org/ml/troubleshooting)
