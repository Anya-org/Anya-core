# Unit Testing Guide [AIR-3][AIS-3][AIT-3][RES-3]

Comprehensive unit testing strategies for Anya-core extensions, ensuring component-level reliability and BIP compliance.

## Overview

Unit tests form the foundation of the Anya-core testing pyramid, providing fast feedback on individual components. Each module must maintain comprehensive unit test coverage with focus on Bitcoin protocol compliance, Web5 integration, and ML system validation.

## Testing Framework

### Core Dependencies
```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"
proptest = "1.0"
bitcoin-test-utils = "0.1"
web5-mock = "0.2"
ml-test-kit = "0.3"
serial_test = "0.9"
```

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use proptest::prelude::*;
    use tokio_test;
    
    // Test module organization
    mod bitcoin_tests;
    mod web5_tests;
    mod ml_tests;
    mod integration_tests;
}
```

## Bitcoin Unit Testing

### Transaction Validation Tests
```rust
#[cfg(test)]
mod bitcoin_transaction_tests {
    use super::*;
    use bitcoin::{Transaction, TxIn, TxOut, OutPoint, Script, Witness};
    use bitcoin::secp256k1::{Secp256k1, SecretKey};
    
    #[test]
    fn test_valid_p2pkh_transaction() {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[1; 32]).unwrap();
        let public_key = secret_key.public_key(&secp);
        
        let tx = Transaction {
            version: 2,
            lock_time: bitcoin::PackedLockTime::ZERO,
            input: vec![create_test_input()],
            output: vec![create_p2pkh_output(&public_key)],
        };
        
        let validator = TransactionValidator::new();
        assert!(validator.validate_structure(&tx).is_ok());
        assert!(validator.validate_signatures(&tx).is_ok());
    }
    
    #[test]
    fn test_invalid_transaction_negative_value() {
        let tx = Transaction {
            version: 2,
            lock_time: bitcoin::PackedLockTime::ZERO,
            input: vec![create_test_input()],
            output: vec![TxOut {
                value: -1, // Invalid negative value
                script_pubkey: Script::new(),
            }],
        };
        
        let validator = TransactionValidator::new();
        assert!(validator.validate_structure(&tx).is_err());
    }
    
    #[test]
    fn test_bip141_witness_validation() {
        let witness_tx = create_segwit_transaction();
        let validator = TransactionValidator::new();
        
        // Test witness structure
        assert!(validator.validate_witness(&witness_tx).is_ok());
        
        // Test witness commitment
        assert!(validator.validate_witness_commitment(&witness_tx).is_ok());
    }
}
```

### Script Testing
```rust
#[cfg(test)]
mod script_tests {
    use super::*;
    use bitcoin::script::{Builder, Instruction};
    
    #[test]
    fn test_p2pkh_script_execution() {
        let pubkey_hash = [0u8; 20];
        let script = Builder::new()
            .push_opcode(opcodes::all::OP_DUP)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(&pubkey_hash)
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();
            
        let interpreter = ScriptInterpreter::new();
        let result = interpreter.execute(&script, &create_test_stack());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_multisig_script_validation() {
        let pubkeys = vec![
            create_test_pubkey(1),
            create_test_pubkey(2),
            create_test_pubkey(3),
        ];
        
        let multisig_script = Builder::new()
            .push_int(2) // 2-of-3 multisig
            .push_slice(&pubkeys[0].serialize())
            .push_slice(&pubkeys[1].serialize())
            .push_slice(&pubkeys[2].serialize())
            .push_int(3)
            .push_opcode(opcodes::all::OP_CHECKMULTISIG)
            .into_script();
            
        let validator = ScriptValidator::new();
        assert!(validator.validate_multisig(&multisig_script, 2, 3).is_ok());
    }
}
```

## Web5 Unit Testing

### DID Testing
```rust
#[cfg(test)]
mod web5_did_tests {
    use super::*;
    use web5::{DID, DidDocument, VerificationMethod};
    
    #[test]
    fn test_did_creation_and_validation() {
        let did = DID::new("web5", "example.com", "alice").unwrap();
        assert_eq!(did.method(), "web5");
        assert_eq!(did.method_specific_id(), "example.com:alice");
        assert!(did.is_valid());
    }
    
    #[test]
    fn test_did_document_resolution() {
        let did = DID::parse("did:web5:example.com:alice").unwrap();
        let document = DidDocument::builder()
            .id(did.clone())
            .verification_method(VerificationMethod::new(
                format!("{}#key-1", did),
                "Ed25519VerificationKey2020",
                did.clone(),
                "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
            ))
            .build();
            
        let resolver = DidResolver::new();
        let resolved = resolver.resolve(&did).unwrap();
        assert_eq!(resolved.id(), &did);
    }
    
    #[test]
    fn test_verifiable_credential_creation() {
        let credential = VerifiableCredential::builder()
            .issuer("did:web5:issuer.com")
            .subject("did:web5:subject.com")
            .credential_type("UniversityDegree")
            .claim("degree", "Bachelor of Science")
            .build()
            .unwrap();
            
        assert!(credential.verify().is_ok());
        assert_eq!(credential.get_claim("degree"), Some("Bachelor of Science"));
    }
}
```

### Web5 Protocol Testing
```rust
#[cfg(test)]
mod web5_protocol_tests {
    use super::*;
    use web5::{DWN, Message, Protocol};
    
    #[test]
    fn test_dwn_message_creation() {
        let message = Message::builder()
            .record_id("bafyreigvp...")
            .data_cid("bafyreihash...")
            .date_created(Utc::now())
            .protocol("https://example.com/protocol")
            .build()
            .unwrap();
            
        assert!(message.is_valid());
        assert_eq!(message.protocol(), Some("https://example.com/protocol"));
    }
    
    #[test]
    fn test_protocol_validation() {
        let protocol = Protocol::new("https://schema.org/SocialMediaPosting");
        let message = create_test_message();
        
        assert!(protocol.validate_message(&message).is_ok());
    }
}
```

## ML Unit Testing

### Model Validation Tests
```rust
#[cfg(test)]
mod ml_model_tests {
    use super::*;
    use ml::{Model, Prediction, TrainingData};
    
    #[test]
    fn test_model_inference() {
        let model = Model::load_from_path("./test-models/simple_classifier.json").unwrap();
        let input = vec![1.0, 2.0, 3.0, 4.0];
        
        let prediction = model.predict(&input).unwrap();
        assert!(prediction.confidence() > 0.0);
        assert!(prediction.confidence() <= 1.0);
    }
    
    #[test]
    fn test_model_training_validation() {
        let training_data = TrainingData::new(
            vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            vec![0, 1]
        );
        
        let mut model = Model::new_classifier(2, 2);
        let result = model.train(&training_data);
        
        assert!(result.is_ok());
        assert!(model.accuracy() > 0.0);
    }
    
    #[test]
    fn test_model_serialization() {
        let model = create_test_model();
        let serialized = model.serialize().unwrap();
        let deserialized = Model::deserialize(&serialized).unwrap();
        
        assert_eq!(model.get_weights(), deserialized.get_weights());
        assert_eq!(model.get_bias(), deserialized.get_bias());
    }
}
```

### Data Processing Tests
```rust
#[cfg(test)]
mod ml_data_tests {
    use super::*;
    use ml::{DataProcessor, Feature, Dataset};
    
    #[test]
    fn test_feature_extraction() {
        let processor = DataProcessor::new();
        let raw_data = "Sample text for feature extraction";
        
        let features = processor.extract_features(raw_data).unwrap();
        assert!(!features.is_empty());
        assert!(features.iter().all(|f| f.value().is_finite()));
    }
    
    #[test]
    fn test_data_normalization() {
        let dataset = Dataset::new(vec![
            vec![1.0, 100.0, 0.1],
            vec![2.0, 200.0, 0.2],
            vec![3.0, 300.0, 0.3],
        ]);
        
        let normalized = dataset.normalize().unwrap();
        
        // Check mean is approximately 0
        let mean = normalized.mean();
        assert!((mean[0]).abs() < 0.1);
        assert!((mean[1]).abs() < 0.1);
        assert!((mean[2]).abs() < 0.1);
    }
}
```

## Property-Based Testing

### Bitcoin Property Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_transaction_serialization_roundtrip(
        version in any::<i32>(),
        lock_time in any::<u32>(),
    ) {
        let tx = Transaction {
            version,
            lock_time: bitcoin::PackedLockTime(lock_time),
            input: vec![],
            output: vec![],
        };
        
        let serialized = bitcoin::consensus::serialize(&tx);
        let deserialized: Transaction = bitcoin::consensus::deserialize(&serialized).unwrap();
        
        prop_assert_eq!(tx, deserialized);
    }
    
    #[test]
    fn test_script_push_data_invariant(data in prop::collection::vec(any::<u8>(), 0..520)) {
        let script = Builder::new().push_slice(&data).into_script();
        let instructions: Vec<_> = script.instructions().collect();
        
        prop_assert_eq!(instructions.len(), 1);
        if let Ok(Instruction::PushBytes(pushed_data)) = &instructions[0] {
            prop_assert_eq!(pushed_data.as_bytes(), &data);
        }
    }
}
```

## Mock and Stub Testing

### Bitcoin Network Mocking
```rust
use mockall::mock;

mock! {
    BitcoinClient {
        fn get_block_height(&self) -> Result<u64, Error>;
        fn get_transaction(&self, txid: &Txid) -> Result<Transaction, Error>;
        fn broadcast_transaction(&self, tx: &Transaction) -> Result<Txid, Error>;
    }
}

#[test]
fn test_transaction_processor_with_mock() {
    let mut mock_client = MockBitcoinClient::new();
    
    mock_client
        .expect_get_block_height()
        .returning(|| Ok(700000));
        
    mock_client
        .expect_broadcast_transaction()
        .returning(|_| Ok(Txid::all_zeros()));
    
    let processor = TransactionProcessor::new(Box::new(mock_client));
    let result = processor.process_transaction(&create_test_tx());
    
    assert!(result.is_ok());
}
```

## Performance Unit Tests

### Benchmark Testing
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_signature_verification(c: &mut Criterion) {
    let tx = create_test_transaction();
    let validator = TransactionValidator::new();
    
    c.bench_function("signature_verification", |b| {
        b.iter(|| validator.verify_signature(black_box(&tx)))
    });
}

fn benchmark_script_execution(c: &mut Criterion) {
    let script = create_complex_script();
    let interpreter = ScriptInterpreter::new();
    
    c.bench_function("script_execution", |b| {
        b.iter(|| interpreter.execute(black_box(&script), &create_test_stack()))
    });
}

criterion_group!(benches, benchmark_signature_verification, benchmark_script_execution);
criterion_main!(benches);
```

## Test Organization Best Practices

### Module Structure
```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    mod unit {
        mod bitcoin;
        mod web5;
        mod ml;
    }
    
    mod helpers;
    mod fixtures;
    mod mocks;
}
```

### Test Helpers
```rust
// tests/helpers/mod.rs
pub fn create_test_transaction() -> Transaction {
    Transaction {
        version: 2,
        lock_time: bitcoin::PackedLockTime::ZERO,
        input: vec![create_test_input()],
        output: vec![create_test_output()],
    }
}

pub fn create_test_private_key() -> PrivateKey {
    PrivateKey::from_wif("cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN87JcbXMTcA").unwrap()
}

pub fn setup_test_environment() -> TestEnvironment {
    TestEnvironment {
        temp_dir: tempfile::tempdir().unwrap(),
        mock_network: MockNetwork::new(),
        test_data: load_test_fixtures(),
    }
}
```

## Error Testing

### Error Condition Coverage
```rust
#[test]
fn test_insufficient_funds_error() {
    let wallet = Wallet::new_with_balance(1000);
    let result = wallet.send_transaction(2000, &Address::from_str("...").unwrap());
    
    match result {
        Err(WalletError::InsufficientFunds { available, required }) => {
            assert_eq!(available, 1000);
            assert_eq!(required, 2000);
        }
        _ => panic!("Expected InsufficientFunds error"),
    }
}

#[test]
fn test_network_timeout_recovery() {
    let mut client = BitcoinClient::new_with_timeout(Duration::from_millis(1));
    let result = client.get_block_height();
    
    assert!(matches!(result, Err(NetworkError::Timeout)));
}
```

## Continuous Integration

### CI Test Configuration
```bash
# Run unit tests with coverage
cargo test --lib --bins --tests --benches

# Generate coverage report
cargo tarpaulin --out Html --output-dir target/coverage/

# Run property tests
cargo test --features proptest-config
```

### Test Quality Metrics
- **Coverage**: Minimum 90% line coverage
- **Performance**: Unit tests under 100ms each
- **Reliability**: Zero flaky tests allowed
- **Maintainability**: Clear test names and documentation

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Proptest Guide](https://altsysrq.github.io/proptest-book/)
- [Mockall Documentation](https://docs.rs/mockall/)
- [Bitcoin Testing Best Practices](https://github.com/bitcoin/bitcoin/blob/master/doc/developer-notes.md#unit-tests)
- [Integration Testing Guide](./integration-testing.md)
- [Performance Testing Guide](./performance-testing.md)

*Last updated: May 30, 2025*
