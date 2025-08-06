# Integration Testing Guide [AIR-3][AIS-3][AIT-3][RES-3]

Comprehensive integration testing for Anya-core extensions, validating cross-system compatibility between Bitcoin networks, Web5 protocols, and ML systems.

## Overview

Integration tests ensure that Anya-core extensions work correctly across system boundaries, testing real-world scenarios with Bitcoin testnet/mainnet, Web5 decentralized networks, and ML model inference pipelines. These tests validate BIP compliance in production-like environments.

## Integration Test Architecture

### Test Categories

- **Bitcoin Network Integration**: Testnet/mainnet connectivity
- **Web5 Protocol Integration**: DWN, DID, and VC interactions
- **ML Pipeline Integration**: Model training and inference workflows
- **Cross-System Integration**: Bitcoin + Web5 + ML combined operations
- **External Service Integration**: Third-party API compatibility
- **Database Integration**: Persistent storage validation

### Test Environment Setup

```bash
# Integration test environment
export BITCOIN_NETWORK=testnet
export WEB5_ENDPOINT=https://dwn.testnet.web5.com
export ML_INFERENCE_URL=http://localhost:8080/predict
export DATABASE_URL=postgresql://test:test@localhost/anya_test
export REDIS_URL=redis://localhost:6379/1
```

## Bitcoin Network Integration

### Testnet Integration Tests

```rust
#[cfg(test)]
mod bitcoin_integration_tests {
    use super::*;
    use bitcoin::{Network, Address, Transaction};
    use bitcoincore_rpc::{Client, Auth, RpcApi};
    
    #[tokio::test]
    #[ignore] // Run with: cargo test --ignored
    async fn test_testnet_transaction_broadcast() {
        let client = Client::new(
            "http://localhost:18332",
            Auth::UserPass("testuser".to_string(), "testpass".to_string())
        ).unwrap();
        
        // Create and sign transaction
        let tx = create_test_transaction_with_real_inputs(&client).await;
        let signed_tx = sign_transaction(&tx, &get_test_private_key()).unwrap();
        
        // Broadcast to testnet
        let txid = client.send_raw_transaction(&signed_tx).unwrap();
        
        // Wait for confirmation
        let confirmed_tx = wait_for_confirmation(&client, &txid, 6).await.unwrap();
        assert_eq!(confirmed_tx.txid(), txid);
    }
    
    #[tokio::test]
    async fn test_bitcoin_address_validation_mainnet() {
        let validator = AddressValidator::new(Network::Bitcoin);
        
        // Test various address formats
        let p2pkh = Address::from_str("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2").unwrap();
        assert!(validator.validate(&p2pkh).is_ok());
        
        let p2sh = Address::from_str("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy").unwrap();
        assert!(validator.validate(&p2sh).is_ok());
        
        let bech32 = Address::from_str("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq").unwrap();
        assert!(validator.validate(&bech32).is_ok());
    }
    
    #[tokio::test]
    async fn test_lightning_network_integration() {
        let ln_client = LightningClient::connect("localhost:10009").await.unwrap();
        
        // Test channel creation
        let channel_request = OpenChannelRequest {
            node_pubkey: "03...".to_string(),
            local_funding_amount: 100_000,
            push_sat: 50_000,
        };
        
        let channel = ln_client.open_channel(channel_request).await.unwrap();
        assert!(channel.is_active());
        
        // Test payment
        let payment_request = PaymentRequest {
            payment_hash: "abc123...".to_string(),
            amount_msat: 1000,
            destination: "03...".to_string(),
        };
        
        let payment_result = ln_client.send_payment(payment_request).await.unwrap();
        assert!(payment_result.is_successful());
    }
}
```

### Block Processing Integration

```rust
#[tokio::test]
async fn test_block_processing_pipeline() {
    let block_processor = BlockProcessor::new(Network::Testnet);
    let bitcoin_client = BitcoinClient::new_testnet().await.unwrap();
    
    // Get latest block
    let block_hash = bitcoin_client.get_best_block_hash().await.unwrap();
    let block = bitcoin_client.get_block(&block_hash).await.unwrap();
    
    // Process block through pipeline
    let processed_block = block_processor.process(&block).await.unwrap();
    
    // Validate processed data
    assert_eq!(processed_block.hash, block.block_hash());
    assert_eq!(processed_block.transactions.len(), block.txdata.len());
    
    // Verify BIP compliance
    for tx in &processed_block.transactions {
        assert!(tx.is_bip_compliant());
    }
}
```

## Web5 Integration Testing

### DID Resolution Integration

```rust
#[cfg(test)]
mod web5_integration_tests {
    use super::*;
    use web5::{DID, DidResolver, DWN, VerifiableCredential};
    
    #[tokio::test]
    async fn test_did_resolution_across_networks() {
        let resolver = DidResolver::new_with_endpoints(vec![
            "https://dwn.testnet.web5.com",
            "https://did.testnet.ion.msidentity.com",
        ]);
        
        // Test Web5 DID resolution
        let web5_did = DID::parse("did:web5:testnet:alice").unwrap();
        let document = resolver.resolve(&web5_did).await.unwrap();
        
        assert_eq!(document.id(), &web5_did);
        assert!(!document.verification_methods().is_empty());
        
        // Test ION DID resolution
        let ion_did = DID::parse("did:ion:EiDyOQbbZAa3aiRzeCkV7LOx3SERjjH93EXoIM3UoN4oWg").unwrap();
        let ion_document = resolver.resolve(&ion_did).await.unwrap();
        
        assert_eq!(ion_document.id(), &ion_did);
    }
    
    #[tokio::test]
    async fn test_dwn_message_flow() {
        let dwn_client = DWNClient::new("https://dwn.testnet.web5.com").await.unwrap();
        let did = create_test_did().await;
        
        // Write message to DWN
        let message = Message::builder()
            .protocol("https://example.com/chat")
            .schema("https://example.com/schemas/message")
            .data(b"Hello, Web5!")
            .build()
            .unwrap();
            
        let write_result = dwn_client.write(&did, message.clone()).await.unwrap();
        assert!(write_result.is_successful());
        
        // Read message from DWN
        let query = QueryBuilder::new()
            .protocol("https://example.com/chat")
            .build();
            
        let messages = dwn_client.query(&did, query).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].data(), message.data());
    }
    
    #[tokio::test]
    async fn test_verifiable_credential_lifecycle() {
        let issuer_did = create_test_did().await;
        let subject_did = create_test_did().await;
        
        // Issue credential
        let credential = VerifiableCredential::builder()
            .issuer(issuer_did.clone())
            .subject(subject_did.clone())
            .credential_type("UniversityDegree")
            .claim("degree", "Bachelor of Computer Science")
            .claim("university", "Test University")
            .build()
            .unwrap();
            
        let signed_credential = credential.sign(&get_issuer_key()).await.unwrap();
        
        // Verify credential
        let verifier = CredentialVerifier::new();
        let verification_result = verifier.verify(&signed_credential).await.unwrap();
        
        assert!(verification_result.is_valid());
        assert_eq!(verification_result.issuer(), &issuer_did);
        assert_eq!(verification_result.subject(), &subject_did);
        
        // Store credential in DWN
        let dwn_client = DWNClient::new("https://dwn.testnet.web5.com").await.unwrap();
        let storage_result = dwn_client.store_credential(&subject_did, &signed_credential).await.unwrap();
        assert!(storage_result.is_successful());
    }
}
```

## ML Pipeline Integration

### Model Training Integration

```rust
#[cfg(test)]
mod ml_integration_tests {
    use super::*;
    use ml::{Model, TrainingPipeline, InferencePipeline};
    
    #[tokio::test]
    async fn test_bitcoin_price_prediction_pipeline() {
        let training_pipeline = TrainingPipeline::new()
            .with_data_source("https://api.coingecko.com/api/v3/coins/bitcoin/market_chart")
            .with_model_type(ModelType::LSTM)
            .with_features(vec!["price", "volume", "market_cap"])
            .with_target("price_next_hour");
            
        // Train model with historical data
        let model = training_pipeline.train().await.unwrap();
        assert!(model.accuracy() > 0.7);
        
        // Test real-time prediction
        let inference_pipeline = InferencePipeline::new(model);
        let current_data = fetch_current_bitcoin_data().await.unwrap();
        
        let prediction = inference_pipeline.predict(&current_data).await.unwrap();
        assert!(prediction.confidence() > 0.5);
        assert!(prediction.value() > 0.0);
    }
    
    #[tokio::test]
    async fn test_transaction_anomaly_detection() {
        let anomaly_detector = AnomalyDetector::load_from_file("models/tx_anomaly.json").unwrap();
        
        // Test with normal transaction
        let normal_tx = create_normal_transaction();
        let normal_score = anomaly_detector.score(&normal_tx).await.unwrap();
        assert!(normal_score < 0.5); // Low anomaly score
        
        // Test with suspicious transaction
        let suspicious_tx = create_suspicious_transaction();
        let suspicious_score = anomaly_detector.score(&suspicious_tx).await.unwrap();
        assert!(suspicious_score > 0.8); // High anomaly score
    }
    
    #[tokio::test]
    async fn test_did_reputation_scoring() {
        let reputation_model = ReputationModel::load_from_file("models/did_reputation.json").unwrap();
        
        // Test established DID
        let established_did = DID::parse("did:web5:established:alice").unwrap();
        let established_score = reputation_model.calculate_score(&established_did).await.unwrap();
        assert!(established_score > 0.7);
        
        // Test new DID
        let new_did = DID::parse("did:web5:new:bob").unwrap();
        let new_score = reputation_model.calculate_score(&new_did).await.unwrap();
        assert!(new_score < 0.5);
    }
}
```

## Cross-System Integration

### Bitcoin + Web5 Integration

```rust
#[tokio::test]
async fn test_bitcoin_web5_identity_verification() {
    // Create Web5 identity
    let did = create_test_did().await;
    let identity_key = generate_identity_key();
    
    // Create Bitcoin address from same key
    let bitcoin_key = derive_bitcoin_key_from_identity(&identity_key);
    let bitcoin_address = Address::p2wpkh(&bitcoin_key.public_key(), Network::Testnet).unwrap();
    
    // Create proof of ownership
    let message = format!("I control Bitcoin address {} with DID {}", bitcoin_address, did);
    let signature = bitcoin_key.sign_message(&message).unwrap();
    
    // Store proof in DWN
    let dwn_client = DWNClient::new("https://dwn.testnet.web5.com").await.unwrap();
    let proof_message = Message::builder()
        .protocol("https://bitcoin.org/address-proof")
        .data(serde_json::to_vec(&AddressProof {
            address: bitcoin_address.to_string(),
            message,
            signature: signature.to_string(),
        }).unwrap())
        .build()
        .unwrap();
        
    let storage_result = dwn_client.write(&did, proof_message).await.unwrap();
    assert!(storage_result.is_successful());
    
    // Verify proof
    let verifier = AddressProofVerifier::new();
    let verification_result = verifier.verify_dwn_proof(&did, &bitcoin_address).await.unwrap();
    assert!(verification_result.is_valid());
}
```

### Web5 + ML Integration

```rust
#[tokio::test]
async fn test_web5_ml_personalization() {
    let did = create_test_did().await;
    let personalization_model = PersonalizationModel::new();
    
    // Store user preferences in DWN
    let preferences = UserPreferences {
        interests: vec!["bitcoin", "defi", "privacy"],
        risk_tolerance: 0.7,
        investment_horizon: "long_term",
    };
    
    let dwn_client = DWNClient::new("https://dwn.testnet.web5.com").await.unwrap();
    let prefs_message = Message::builder()
        .protocol("https://anya.org/user-preferences")
        .data(serde_json::to_vec(&preferences).unwrap())
        .build()
        .unwrap();
        
    dwn_client.write(&did, prefs_message).await.unwrap();
    
    // Generate personalized recommendations
    let user_data = dwn_client.read_user_data(&did).await.unwrap();
    let recommendations = personalization_model.generate_recommendations(&user_data).await.unwrap();
    
    assert!(!recommendations.is_empty());
    assert!(recommendations.iter().any(|r| r.category == "bitcoin"));
}
```

## Database Integration

### Persistent Storage Tests

```rust
#[tokio::test]
async fn test_database_transaction_consistency() {
    let db_pool = create_test_database_pool().await;
    let transaction_store = TransactionStore::new(db_pool.clone());
    
    // Start database transaction
    let mut db_tx = db_pool.begin().await.unwrap();
    
    // Store Bitcoin transaction
    let bitcoin_tx = create_test_transaction();
    transaction_store.store_bitcoin_transaction(&mut db_tx, &bitcoin_tx).await.unwrap();
    
    // Store Web5 message
    let web5_message = create_test_message();
    transaction_store.store_web5_message(&mut db_tx, &web5_message).await.unwrap();
    
    // Store ML prediction
    let prediction = create_test_prediction();
    transaction_store.store_ml_prediction(&mut db_tx, &prediction).await.unwrap();
    
    // Commit transaction
    db_tx.commit().await.unwrap();
    
    // Verify all data is stored consistently
    let stored_bitcoin_tx = transaction_store.get_bitcoin_transaction(&bitcoin_tx.txid()).await.unwrap();
    assert_eq!(stored_bitcoin_tx.txid(), bitcoin_tx.txid());
    
    let stored_web5_message = transaction_store.get_web5_message(&web5_message.id()).await.unwrap();
    assert_eq!(stored_web5_message.id(), web5_message.id());
    
    let stored_prediction = transaction_store.get_ml_prediction(&prediction.id()).await.unwrap();
    assert_eq!(stored_prediction.id(), prediction.id());
}
```

## Performance Integration Tests

### Load Testing

```rust
#[tokio::test]
async fn test_concurrent_transaction_processing() {
    let processor = TransactionProcessor::new();
    let transaction_count = 1000;
    
    let transactions: Vec<_> = (0..transaction_count)
        .map(|_| create_test_transaction())
        .collect();
    
    let start_time = Instant::now();
    
    // Process transactions concurrently
    let results: Vec<_> = futures::future::join_all(
        transactions.iter().map(|tx| processor.process_transaction(tx))
    ).await;
    
    let duration = start_time.elapsed();
    
    // Verify all transactions processed successfully
    for result in results {
        assert!(result.is_ok());
    }
    
    // Verify performance requirements
    let throughput = transaction_count as f64 / duration.as_secs_f64();
    assert!(throughput > 100.0); // At least 100 TPS
}
```

## Test Environment Management

### Docker Test Environment

```yaml
# docker-compose.test.yml
version: '3.8'
services:
  bitcoin-testnet:
    image: ruimarinho/bitcoin-core:latest
    command: >
      bitcoind
      -testnet=1
      -rpcuser=testuser
      -rpcpassword=testpass
      -rpcallowip=0.0.0.0/0
      -server=1
    ports:
      - "18332:18332"
      
  postgres:
    image: postgres:14
    environment:
      POSTGRES_DB: anya_test
      POSTGRES_USER: test
      POSTGRES_PASSWORD: test
    ports:
      - "5432:5432"
      
  redis:
    image: redis:alpine
    ports:
      - "6379:6379"
      
  web5-dwn:
    image: tbd54566975/dwn-server:latest
    ports:
      - "3000:3000"
```

### Test Setup and Teardown

```rust
use testcontainers::{Docker, Container, Image};

pub struct TestEnvironment {
    bitcoin_container: Container<'static, Docker, BitcoinImage>,
    postgres_container: Container<'static, Docker, PostgresImage>,
    redis_container: Container<'static, Docker, RedisImage>,
}

impl TestEnvironment {
    pub async fn new() -> Self {
        let docker = Docker::default();
        
        let bitcoin_container = docker.run(BitcoinImage::default());
        let postgres_container = docker.run(PostgresImage::default());
        let redis_container = docker.run(RedisImage::default());
        
        // Wait for services to be ready
        wait_for_bitcoin_ready(&bitcoin_container).await;
        wait_for_postgres_ready(&postgres_container).await;
        wait_for_redis_ready(&redis_container).await;
        
        Self {
            bitcoin_container,
            postgres_container,
            redis_container,
        }
    }
    
    pub fn bitcoin_rpc_url(&self) -> String {
        format!("http://localhost:{}", self.bitcoin_container.get_host_port(18332))
    }
    
    pub fn postgres_url(&self) -> String {
        format!("postgresql://test:test@localhost:{}/anya_test", 
                self.postgres_container.get_host_port(5432))
    }
}
```

## Continuous Integration

### CI Integration Test Pipeline

```yaml
# .github/workflows/integration-tests.yml
name: Integration Tests
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  integration-tests:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:14
        env:
          POSTGRES_PASSWORD: test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Start Bitcoin Testnet
        run: |
          docker run -d --name bitcoin-testnet \
            -p 18332:18332 \
            ruimarinho/bitcoin-core:latest \
            bitcoind -testnet=1 -rpcuser=test -rpcpassword=test
      
      - name: Run Integration Tests
        run: cargo test --features integration-tests --ignored
        env:
          DATABASE_URL: postgresql://postgres:test@localhost/test
          REDIS_URL: redis://localhost:6379
          BITCOIN_RPC_URL: http://test:test@localhost:18332
```

## Best Practices

### Test Isolation

1. **Clean State**: Each test starts with a fresh environment
2. **Independent**: Tests don't depend on execution order
3. **Idempotent**: Tests can be run multiple times safely
4. **Atomic**: Each test validates one integration scenario

### Error Handling

```rust
#[tokio::test]
async fn test_network_failure_recovery() {
    let client = BitcoinClient::new_with_retries(3);
    
    // Simulate network failure
    let result = client.get_block_height_with_simulated_failure().await;
    
    match result {
        Ok(height) => assert!(height > 0),
        Err(e) => assert!(e.is_retryable()),
    }
}
```

### Test Data Management

```rust
lazy_static! {
    static ref TEST_DATA: TestDataManager = TestDataManager::new();
}

impl TestDataManager {
    pub fn get_test_transaction(&self, scenario: &str) -> Transaction {
        match scenario {
            "valid_p2pkh" => self.load_fixture("valid_p2pkh_tx.json"),
            "multisig_2_of_3" => self.load_fixture("multisig_2_of_3_tx.json"),
            _ => panic!("Unknown test scenario: {}", scenario),
        }
    }
}
```

## Resources

- [Bitcoin Testnet Guide](https://developer.bitcoin.org/examples/testing.html)
- [Web5 Testing Documentation](https://developer.tbd.website/docs/web5/build/test/)
- [Testcontainers Rust](https://docs.rs/testcontainers/)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Unit Testing Guide](./unit-testing.md)
- [Performance Testing Guide](./performance-testing.md)

*Last updated: June 7, 2025*
