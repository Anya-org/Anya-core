# Testing Guide

**AI Labeling**: This documentation is AI-generated with technical review and validation.

**Date**: May 30, 2025

## Overview

Comprehensive testing guide for Anya Core, covering unit testing, integration testing, performance testing, and security testing across Bitcoin, Web5, and ML systems.

## Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Environment Setup](#test-environment-setup)
- [Unit Testing](#unit-testing)
- [Integration Testing](#integration-testing)
- [Performance Testing](#performance-testing)
- [Security Testing](#security-testing)
- [Test Data Management](#test-data-management)
- [Continuous Integration](#continuous-integration)

## Testing Philosophy

Our testing approach follows these principles:

1. **Comprehensive Coverage**: >80% code coverage requirement
2. **Fast Feedback**: Unit tests run in <30 seconds
3. **Reliable**: Tests are deterministic and reproducible
4. **Maintainable**: Tests are easy to understand and modify
5. **Security-First**: All security-critical code has dedicated tests

## Test Environment Setup

### Prerequisites

```bash
# Install testing dependencies
cargo install cargo-tarpaulin  # Coverage reporting
cargo install cargo-mutagen    # Mutation testing
cargo install criterion        # Benchmarking

# Install Bitcoin testing tools
sudo apt-get install bitcoind  # Bitcoin Core for testing
```

### Test Configuration

```toml
# Cargo.toml test configuration
[dev-dependencies]
tokio-test = "0.4"
proptest = "1.4"
criterion = "0.5"
mock_instant = "0.3"
tempfile = "3.8"
```

### Environment Variables

```bash
# Set test environment variables
export RUST_TEST_THREADS=1        # For deterministic tests
export BITCOIN_RPC_URL="http://localhost:18443"
export BITCOIN_RPC_USER="test"
export BITCOIN_RPC_PASS="test"
export WEB5_TEST_MODE="true"
```

## Unit Testing

### Test Structure

```rust
// src/bitcoin/wallet/tests.rs
use super::*;
use proptest::prelude::*;
use tokio_test;

#[cfg(test)]
mod wallet_tests {
    use super::*;
    
    #[test]
    fn test_address_generation() {
        let wallet = Wallet::new_test();
        let address = wallet.generate_address().unwrap();
        
        assert!(address.is_valid());
        assert_eq!(address.network(), Network::Testnet);
    }
    
    #[tokio::test]
    async fn test_balance_calculation() {
        let mut wallet = Wallet::new_test();
        wallet.add_utxo(create_test_utxo(100_000)).await;
        
        let balance = wallet.get_balance().await.unwrap();
        assert_eq!(balance, 100_000);
    }
    
    // Property-based testing
    proptest! {
        #[test]
        fn test_transaction_serialization(
            inputs in vec(any::<TxInput>(), 1..10),
            outputs in vec(any::<TxOutput>(), 1..10)
        ) {
            let tx = Transaction::new(inputs, outputs);
            let serialized = tx.serialize();
            let deserialized = Transaction::deserialize(&serialized).unwrap();
            
            prop_assert_eq!(tx, deserialized);
        }
    }
}
```

### Test Utilities

```rust
// tests/common/mod.rs
pub mod bitcoin_test_utils {
    use bitcoin::*;
    
    pub fn create_test_utxo(value: u64) -> Utxo {
        Utxo {
            txid: Txid::default(),
            vout: 0,
            value,
            script_pubkey: Script::new(),
        }
    }
    
    pub fn create_test_transaction() -> Transaction {
        Transaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![],
        }
    }
}

pub mod web5_test_utils {
    use web5::*;
    
    pub fn create_test_did() -> Did {
        Did::new("did:test:123456789abcdefghi").unwrap()
    }
    
    pub fn create_test_credential() -> VerifiableCredential {
        VerifiableCredential::new()
            .issuer("did:test:issuer")
            .subject("did:test:subject")
            .build()
            .unwrap()
    }
}
```

### Mocking and Test Doubles

```rust
// Mock Bitcoin RPC client
#[cfg(test)]
pub struct MockBitcoinRpc {
    responses: HashMap<String, serde_json::Value>,
}

impl MockBitcoinRpc {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }
    
    pub fn expect_call(&mut self, method: &str, response: serde_json::Value) {
        self.responses.insert(method.to_string(), response);
    }
}

#[async_trait]
impl BitcoinRpc for MockBitcoinRpc {
    async fn call(&self, method: &str, params: &[serde_json::Value]) -> Result<serde_json::Value> {
        self.responses.get(method)
            .cloned()
            .ok_or_else(|| anyhow!("Unexpected RPC call: {}", method))
    }
}
```

## Integration Testing

### Bitcoin Integration Tests

```rust
// tests/integration/bitcoin_integration.rs
use anya_core::bitcoin::*;
use bitcoin_test_utils::*;

#[tokio::test]
async fn test_full_transaction_flow() {
    // Setup test environment
    let node = start_bitcoin_regtest().await;
    let wallet = Wallet::connect(&node).await.unwrap();
    
    // Generate initial funds
    let address = wallet.generate_address().await.unwrap();
    node.generate_to_address(101, &address).await.unwrap();
    
    // Wait for funds to mature
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Create and broadcast transaction
    let recipient = Address::from_str("bcrt1qtest...").unwrap();
    let amount = 50_000_000; // 0.5 BTC
    
    let tx = wallet.create_transaction(&recipient, amount).await.unwrap();
    let txid = wallet.broadcast_transaction(tx).await.unwrap();
    
    // Verify transaction in mempool
    let mempool = node.get_raw_mempool().await.unwrap();
    assert!(mempool.contains(&txid));
    
    // Mine transaction
    node.generate_blocks(1).await.unwrap();
    
    // Verify transaction is confirmed
    let tx_info = node.get_transaction(&txid).await.unwrap();
    assert_eq!(tx_info.confirmations, 1);
}

#[tokio::test]
async fn test_lightning_channel_management() {
    let alice_node = LightningNode::new("alice").await.unwrap();
    let bob_node = LightningNode::new("bob").await.unwrap();
    
    // Connect nodes
    alice_node.connect_peer(&bob_node.pubkey(), &bob_node.address()).await.unwrap();
    
    // Open channel
    let channel_id = alice_node.open_channel(
        &bob_node.pubkey(),
        1_000_000, // 0.01 BTC
        500_000,   // 0.005 BTC push amount
    ).await.unwrap();
    
    // Wait for channel to be active
    alice_node.wait_for_channel_active(&channel_id).await.unwrap();
    
    // Send payment
    let invoice = bob_node.create_invoice(100_000, "test payment").await.unwrap();
    let payment_hash = alice_node.pay_invoice(&invoice).await.unwrap();
    
    // Verify payment
    assert!(alice_node.payment_succeeded(&payment_hash).await.unwrap());
}
```

### Web5 Integration Tests

```rust
// tests/integration/web5_integration.rs
use anya_core::web5::*;

#[tokio::test]
async fn test_did_resolution_flow() {
    let resolver = DidResolver::new().await;
    
    // Create DID
    let did_document = DidDocument::builder()
        .id("did:test:123456789abcdefghi")
        .verification_method(VerificationMethod::new())
        .build()
        .unwrap();
    
    // Register DID
    resolver.register(&did_document).await.unwrap();
    
    // Resolve DID
    let resolved = resolver.resolve(&did_document.id).await.unwrap();
    assert_eq!(resolved.id, did_document.id);
}

#[tokio::test]
async fn test_dwn_data_storage() {
    let dwn = DecentralizedWebNode::new().await;
    
    // Store data
    let data = b"test data";
    let record_id = dwn.store_data(data).await.unwrap();
    
    // Retrieve data
    let retrieved = dwn.get_data(&record_id).await.unwrap();
    assert_eq!(retrieved, data);
    
    // Update data
    let new_data = b"updated test data";
    dwn.update_data(&record_id, new_data).await.unwrap();
    
    let updated = dwn.get_data(&record_id).await.unwrap();
    assert_eq!(updated, new_data);
}
```

### ML System Integration Tests

```rust
// tests/integration/ml_integration.rs
use anya_core::ml::*;

#[tokio::test]
async fn test_agent_coordination() {
    let coordinator = AgentCoordinator::new().await;
    
    // Register agents
    let trading_agent = TradingAgent::new().await;
    let analysis_agent = AnalysisAgent::new().await;
    
    coordinator.register_agent(Box::new(trading_agent)).await.unwrap();
    coordinator.register_agent(Box::new(analysis_agent)).await.unwrap();
    
    // Send task to coordinator
    let task = Task::new("analyze_market_trends");
    let result = coordinator.execute_task(task).await.unwrap();
    
    assert!(result.is_success());
}

#[tokio::test]
async fn test_model_inference_pipeline() {
    let pipeline = InferencePipeline::new().await;
    
    // Load model
    pipeline.load_model("price_prediction_v1").await.unwrap();
    
    // Prepare input data
    let market_data = MarketData {
        price: 50000.0,
        volume: 1000000.0,
        timestamp: Utc::now(),
    };
    
    // Run inference
    let prediction = pipeline.predict(&market_data).await.unwrap();
    
    assert!(prediction.confidence > 0.5);
}
```

## Performance Testing

### Benchmark Configuration

```rust
// benches/bitcoin_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use anya_core::bitcoin::*;

fn benchmark_signature_verification(c: &mut Criterion) {
    let (private_key, public_key) = generate_keypair();
    let message = b"benchmark message";
    let signature = sign_message(&private_key, message);
    
    c.bench_function("signature_verification", |b| {
        b.iter(|| {
            verify_signature(
                black_box(&signature),
                black_box(message),
                black_box(&public_key)
            )
        })
    });
}

fn benchmark_transaction_validation(c: &mut Criterion) {
    let tx = create_benchmark_transaction();
    let utxo_set = create_benchmark_utxo_set();
    
    c.bench_function("transaction_validation", |b| {
        b.iter(|| {
            validate_transaction(black_box(&tx), black_box(&utxo_set))
        })
    });
}

criterion_group!(
    bitcoin_benches,
    benchmark_signature_verification,
    benchmark_transaction_validation
);
criterion_main!(bitcoin_benches);
```

### Load Testing

```rust
// tests/load/transaction_load_test.rs
use tokio::time::{Duration, Instant};
use futures::future::join_all;

#[tokio::test]
async fn test_concurrent_transaction_processing() {
    let processor = TransactionProcessor::new().await;
    let num_transactions = 1000;
    let concurrency = 10;
    
    let start = Instant::now();
    
    // Create concurrent transaction processing tasks
    let tasks: Vec<_> = (0..concurrency)
        .map(|_| {
            let processor = processor.clone();
            tokio::spawn(async move {
                for _ in 0..(num_transactions / concurrency) {
                    let tx = create_test_transaction();
                    processor.process_transaction(tx).await.unwrap();
                }
            })
        })
        .collect();
    
    // Wait for all tasks to complete
    join_all(tasks).await;
    
    let duration = start.elapsed();
    let tps = num_transactions as f64 / duration.as_secs_f64();
    
    println!("Processed {} transactions in {:?} ({:.2} TPS)", 
             num_transactions, duration, tps);
    
    // Assert minimum performance requirements
    assert!(tps > 100.0, "Transaction processing rate too low: {:.2} TPS", tps);
}
```

### Memory and Resource Testing

```rust
// tests/performance/memory_test.rs
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

#[test]
fn test_memory_usage() {
    let initial_memory = ALLOCATED.load(Ordering::SeqCst);
    
    {
        let wallet = Wallet::new_test();
        // Perform operations
        for _ in 0..1000 {
            wallet.generate_address().unwrap();
        }
        
        let peak_memory = ALLOCATED.load(Ordering::SeqCst);
        let memory_used = peak_memory - initial_memory;
        
        // Assert reasonable memory usage
        assert!(memory_used < 10_000_000, "Memory usage too high: {} bytes", memory_used);
    }
    
    // Force garbage collection
    std::hint::black_box(());
    
    let final_memory = ALLOCATED.load(Ordering::SeqCst);
    let leaked_memory = final_memory - initial_memory;
    
    assert!(leaked_memory < 1000, "Memory leak detected: {} bytes", leaked_memory);
}
```

## Security Testing

### Cryptographic Testing

```rust
// tests/security/crypto_tests.rs
use anya_core::security::*;
use proptest::prelude::*;

#[test]
fn test_constant_time_comparison() {
    let secret1 = b"secret_value_123";
    let secret2 = b"secret_value_123";
    let different = b"different_value_";
    
    // Test that equal values return true
    assert!(constant_time_eq(secret1, secret2));
    
    // Test that different values return false
    assert!(!constant_time_eq(secret1, different));
    
    // Timing attack resistance test (simplified)
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        constant_time_eq(secret1, secret2);
    }
    let equal_time = start.elapsed();
    
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        constant_time_eq(secret1, different);
    }
    let different_time = start.elapsed();
    
    // Times should be similar (within 10% difference)
    let ratio = equal_time.as_nanos() as f64 / different_time.as_nanos() as f64;
    assert!(ratio > 0.9 && ratio < 1.1, "Timing difference detected: {:.2}", ratio);
}

proptest! {
    #[test]
    fn test_signature_security_properties(
        message1 in any::<Vec<u8>>(),
        message2 in any::<Vec<u8>>(),
        private_key in any::<[u8; 32]>()
    ) {
        prop_assume!(message1 != message2);
        
        let signature1 = sign_message(&private_key, &message1);
        let signature2 = sign_message(&private_key, &message2);
        
        // Different messages should produce different signatures
        prop_assert_ne!(signature1, signature2);
        
        // Signatures should verify correctly
        prop_assert!(verify_signature(&signature1, &message1, &private_key));
        prop_assert!(verify_signature(&signature2, &message2, &private_key));
        
        // Cross-verification should fail
        prop_assert!(!verify_signature(&signature1, &message2, &private_key));
        prop_assert!(!verify_signature(&signature2, &message1, &private_key));
    }
}
```

### Fuzzing Tests

```rust
// fuzz/fuzz_targets/transaction_parser.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use anya_core::bitcoin::Transaction;

fuzz_target!(|data: &[u8]| {
    // Test transaction parsing with random input
    if let Ok(tx) = Transaction::deserialize(data) {
        // If parsing succeeds, re-serialization should work
        let serialized = tx.serialize();
        let reparsed = Transaction::deserialize(&serialized).unwrap();
        assert_eq!(tx, reparsed);
    }
});
```

## Test Data Management

### Test Data Generation

```rust
// tests/data/generators.rs
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Dummy)]
pub struct TestMarketData {
    #[dummy(faker = "1000.0..100000.0")]
    pub price: f64,
    
    #[dummy(faker = "0.0..10000000.0")]
    pub volume: f64,
    
    #[dummy(faker = "chrono::Utc::now()")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    #[dummy(faker = "vec![String; 1..10]")]
    pub symbols: Vec<String>,
}

impl TestMarketData {
    pub fn generate_batch(count: usize) -> Vec<Self> {
        (0..count).map(|_| Faker.fake()).collect()
    }
}
```

### Test Database Management

```rust
// tests/database/mod.rs
use sqlx::{PgPool, Row};
use testcontainers::*;

pub struct TestDatabase {
    _container: Container<'static, PostgresImage>,
    pub pool: PgPool,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let docker = clients::Cli::default();
        let container = docker.run(images::postgres::Postgres::default());
        
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            container.get_host_port_ipv4(5432)
        );
        
        let pool = PgPool::connect(&connection_string).await.unwrap();
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        Self {
            _container: container,
            pool,
        }
    }
    
    pub async fn seed_test_data(&self) -> Result<()> {
        sqlx::query("INSERT INTO test_data (value) VALUES ('test')")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
```

## Continuous Integration

### GitHub Actions Configuration

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      
      bitcoin:
        image: bitcoin/bitcoin:latest
        options: >-
          --health-cmd "bitcoin-cli -regtest getblockchaininfo"
          --health-interval 30s
          --health-timeout 10s
          --health-retries 5
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: |
        cargo test --all-features
        cargo test --release --all-features
    
    - name: Run integration tests
      run: cargo test --test integration --all-features
    
    - name: Generate coverage report
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out xml
    
    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        file: ./cobertura.xml
```

### Test Reporting

```rust
// Custom test reporter
use std::io::Write;

pub struct TestReporter {
    passed: usize,
    failed: usize,
    start_time: std::time::Instant,
}

impl TestReporter {
    pub fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            start_time: std::time::Instant::now(),
        }
    }
    
    pub fn test_passed(&mut self, test_name: &str) {
        self.passed += 1;
        println!("✓ {}", test_name);
    }
    
    pub fn test_failed(&mut self, test_name: &str, error: &str) {
        self.failed += 1;
        println!("✗ {} - {}", test_name, error);
    }
    
    pub fn generate_report(&self) -> String {
        let duration = self.start_time.elapsed();
        format!(
            "Test Results: {} passed, {} failed ({:.2}s)",
            self.passed,
            self.failed,
            duration.as_secs_f64()
        )
    }
}
```

## Best Practices

### Test Organization

1. **Arrange-Act-Assert**: Structure tests clearly
2. **Descriptive Names**: Test names should describe what they test
3. **Independent Tests**: Tests should not depend on each other
4. **Fast Tests**: Unit tests should run quickly
5. **Realistic Data**: Use realistic test data

### Error Testing

```rust
#[test]
fn test_invalid_transaction_handling() {
    let invalid_tx = Transaction::new();
    
    match validate_transaction(&invalid_tx) {
        Err(ValidationError::InvalidTransaction(_)) => {
            // Expected error
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
        Ok(_) => panic!("Expected validation to fail"),
    }
}
```

### Test Maintenance

- Update tests when code changes
- Remove obsolete tests
- Keep test data fresh
- Monitor test performance
- Review test coverage regularly

## Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Property-Based Testing](https://github.com/AltSysrq/proptest)
- [Criterion Benchmarking](https://github.com/bheisler/criterion.rs)
- [Testcontainers for Rust](https://github.com/testcontainers/testcontainers-rs)

---

This testing guide ensures comprehensive quality assurance across all Anya Core systems and is maintained by the development team.
