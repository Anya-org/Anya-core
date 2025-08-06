# Testing Framework [AIR-3][AIS-3][AIT-3][RES-3]

Comprehensive testing guide for Anya-core extensions, ensuring reliability, security, and BIP compliance across Bitcoin, Web5, and ML integrations.

## Overview

The Anya-core testing framework provides multi-layered validation for extensions, from unit tests to integration testing with Bitcoin networks and Web5 protocols. All tests must maintain BIP compliance and follow security best practices.

## Testing Architecture

### Test Categories

- **Unit Tests**: Component-level validation
- **Integration Tests**: Cross-system compatibility
- **Performance Tests**: Load and stress testing
- **Security Tests**: Vulnerability and compliance validation
- **Network Tests**: Bitcoin testnet/mainnet integration
- **ML Tests**: Machine learning model validation

### Testing Stack

```rust
// Example test structure
#[cfg(test)]
mod tests {
    use super::*;
    use anya_core::{bitcoin, web5, ml};
    use bitcoin::Network;
    
    #[tokio::test]
    async fn test_bitcoin_transaction_validation() {
        let network = Network::Testnet;
        let validator = bitcoin::TransactionValidator::new(network);
        
        // Test BIP-compliant transaction
        let tx = create_test_transaction();
        assert!(validator.validate(&tx).await.is_ok());
    }
}
```

## Test Environment Setup

### Prerequisites

```bash
# Install test dependencies
cargo install cargo-nextest
cargo install cargo-audit
npm install -g @bitcoin/test-utils
```

### Configuration

```toml
# Cargo.toml test configuration
[dev-dependencies]
tokio-test = "0.4"
bitcoin-test-utils = "0.1"
web5-test-kit = "0.3"
ml-test-framework = "0.2"
proptest = "1.0"
```

### Environment Variables

```bash
export BITCOIN_NETWORK=testnet
export WEB5_TEST_MODE=true
export ML_MODEL_PATH=./test-models/
export ANYA_LOG_LEVEL=debug
```

## Running Tests

### Quick Test Suite

```bash
# Run all tests
cargo nextest run

# Run specific category
cargo test --features bitcoin-tests
cargo test --features web5-tests
cargo test --features ml-tests
```

### Comprehensive Testing

```bash
# Full test suite with coverage
cargo test --all-features
cargo tarpaulin --out Html

# Security audit
cargo audit
cargo clippy -- -D warnings
```

### Network Testing

```bash
# Bitcoin testnet integration
cargo test --features testnet-integration

# Mainnet validation (read-only)
cargo test --features mainnet-validation
```

## Test Data Management

### Bitcoin Test Data

```rust
// Test transaction creation
pub fn create_test_transaction() -> Transaction {
    Transaction {
        version: 2,
        lock_time: PackedLockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            script_sig: Script::new(),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: 50_000,
            script_pubkey: Script::new_p2pkh(&PublicKeyHash::all_zeros()),
        }],
    }
}
```

### Web5 Test Fixtures

```typescript
// DID test data
export const testDID = {
  id: "did:web5:test:alice",
  verificationMethod: [{
    id: "#key-1",
    type: "Ed25519VerificationKey2020",
    controller: "did:web5:test:alice",
    publicKeyMultibase: "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  }]
};
```

### ML Test Models

```python
# Lightweight test model
def create_test_model():
    return {
        'model_type': 'simple_classifier',
        'features': ['input_size', 'output_size'],
        'weights': np.random.randn(10, 10),
        'bias': np.zeros(10)
    }
```

## Test Organization

### Directory Structure

```
tests/
├── unit/
│   ├── bitcoin/
│   ├── web5/
│   └── ml/
├── integration/
│   ├── cross_system/
│   └── end_to_end/
├── performance/
│   ├── load/
│   └── stress/
├── security/
│   ├── vulnerability/
│   └── compliance/
└── fixtures/
    ├── bitcoin/
    ├── web5/
    └── ml/
```

### Test Naming Convention

```rust
// Format: test_[component]_[scenario]_[expected_outcome]
#[test]
fn test_bitcoin_validator_invalid_signature_returns_error() {
    // Test implementation
}

#[test]
fn test_web5_did_resolution_valid_did_returns_document() {
    // Test implementation
}
```

## Continuous Integration

### GitHub Actions Integration

```yaml
# .github/workflows/test.yml
name: Test Suite
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run Tests
        run: |
          cargo nextest run --all-features
          cargo audit
```

### Test Coverage Requirements

- **Unit Tests**: Minimum 90% coverage
- **Integration Tests**: All critical paths covered
- **Security Tests**: All attack vectors validated
- **Performance Tests**: Baseline metrics established

## Best Practices

### Test Quality

1. **Isolation**: Each test should be independent
2. **Determinism**: Tests must produce consistent results
3. **Speed**: Unit tests should complete in milliseconds
4. **Clarity**: Test names should describe the scenario
5. **Coverage**: Aim for comprehensive edge case testing

### Security Testing

```rust
#[test]
fn test_private_key_never_logged() {
    let key = PrivateKey::generate();
    let log_output = capture_logs(|| {
        process_transaction_with_key(&key);
    });
    assert!(!log_output.contains(&key.to_string()));
}
```

### Performance Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_bitcoin_validation(c: &mut Criterion) {
    c.bench_function("bitcoin_validation", |b| {
        b.iter(|| validate_transaction(black_box(&test_tx)))
    });
}
```

## Troubleshooting

### Common Issues

1. **Network Timeouts**: Increase timeout values for testnet operations
2. **Resource Limits**: Ensure sufficient memory for ML model tests
3. **Race Conditions**: Use proper synchronization in async tests
4. **Flaky Tests**: Implement retry mechanisms for network-dependent tests

### Debug Tools

```bash
# Verbose test output
RUST_LOG=debug cargo test -- --nocapture

# Test specific module
cargo test bitcoin::validator --features debug-output

# Memory profiling
cargo test --features memory-profiling
```

## Resources

- [Unit Testing Guide](./unit-testing.md)
- [Integration Testing Guide](./integration-testing.md)
- [Performance Testing Guide](./performance-testing.md)
- [Bitcoin Test Networks](https://developer.bitcoin.org/examples/testing.html)
- [Web5 Testing Best Practices](https://developer.tbd.website/docs/web5/build/test/)

*Last updated: June 7, 2025*
