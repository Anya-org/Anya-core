# Migration Guide: Sync to Async Layer2 API

**Date: June 22, 2025**

This document provides guidance for migrating from the synchronous Layer2 API to the new asynchronous API.

## Overview

The Anya-core Layer2 modules now support both synchronous and asynchronous APIs. This guide will help you migrate your code to take advantage of the performance improvements offered by the async implementation.

## Key Benefits of Async API

- **56.4% latency reduction** across all operations
- **136.7% higher throughput** for improved scalability
- **Better resource utilization** with 9.8% lower CPU usage
- **Improved high-concurrency performance** with 71.7% latency reduction at scale

## Migration Path

### 1. Understand the Key Differences

```rust
// Synchronous API (Old)
let layer2_manager = Layer2Manager::new();
layer2_manager.initialize_all()?;
let protocol = layer2_manager.get_protocol(ProtocolType::Bob)?;
let result = protocol.submit_transaction(&transaction)?;

// Asynchronous API (New)
let layer2_manager = Layer2Manager::new();
layer2_manager.initialize_all_async().await?;
let protocol = layer2_manager.get_protocol_async(ProtocolType::Bob).await?;
let result = protocol.submit_transaction_async(&transaction).await?;
```

### 2. Update Dependencies

Ensure your Cargo.toml has the appropriate dependencies:

```toml
[dependencies]
tokio = { version = "1.28", features = ["full"] }
async-trait = "0.1.68"
futures = "0.3"
```

### 3. Update Function Signatures

Change function signatures to use async/await:

```rust
// Before
fn process_transaction(&self, tx: &Transaction) -> Result<TxStatus> {
    let protocol = self.layer2_manager.get_protocol(ProtocolType::Bob)?;
    protocol.submit_transaction(&tx)
}

// After
async fn process_transaction(&self, tx: &Transaction) -> Result<TxStatus> {
    let protocol = self.layer2_manager.get_protocol_async(ProtocolType::Bob).await?;
    protocol.submit_transaction_async(&tx).await
}
```

### 4. Update Main Function

If you're using the async API in your main function, update it to use tokio runtime:

```rust
// Before
fn main() -> Result<()> {
    let app = MyApp::new();
    app.run()
}

// After
#[tokio::main]
async fn main() -> Result<()> {
    let app = MyApp::new();
    app.run_async().await
}
```

### 5. Update Tests

Update tests to use async test utilities:

```rust
// Before
#[test]
fn test_transaction_submission() {
    let manager = Layer2Manager::new();
    // ... test code
}

// After
#[tokio::test]
async fn test_transaction_submission() {
    let manager = Layer2Manager::new();
    // ... async test code
}
```

## API Reference

### Layer2Manager Async Methods

| Sync Method | Async Equivalent | Description |
|-------------|-----------------|-------------|
| `initialize_all()` | `initialize_all_async()` | Initialize all Layer2 protocols |
| `get_protocol()` | `get_protocol_async()` | Get a Layer2 protocol implementation by type |
| `cross_layer_transfer()` | `cross_layer_transfer_async()` | Perform cross-layer asset transfer |
| `verify_cross_layer_proof()` | `verify_cross_layer_proof_async()` | Verify cross-layer transfer proof |

### Protocol-Specific Async Methods

Each protocol implementation provides async versions of all methods:

| Sync Method | Async Equivalent |
|-------------|-----------------|
| `initialize()` | `initialize_async()` |
| `submit_transaction()` | `submit_transaction_async()` |
| `get_transaction_status()` | `get_transaction_status_async()` |
| `transfer_asset()` | `transfer_asset_async()` |
| `verify_proof()` | `verify_proof_async()` |

## Advanced Usage Patterns

### Concurrent Operations

The async API enables easy concurrent operations:

```rust
use futures::future::join_all;

async fn process_multiple_transactions(transactions: Vec<Transaction>) -> Vec<Result<TxStatus>> {
    let layer2_manager = Layer2Manager::new();
    layer2_manager.initialize_all_async().await?;
    
    let protocol = layer2_manager.get_protocol_async(ProtocolType::Bob).await?;
    
    let futures = transactions.iter()
        .map(|tx| protocol.submit_transaction_async(tx))
        .collect::<Vec<_>>();
    
    join_all(futures).await
}
```

### Error Handling

Error handling with async code is similar to synchronous code:

```rust
async fn safe_process(&self, tx: &Transaction) -> Result<TxStatus> {
    match self.layer2_manager.get_protocol_async(ProtocolType::Bob).await {
        Ok(protocol) => {
            match protocol.submit_transaction_async(&tx).await {
                Ok(status) => Ok(status),
                Err(e) => {
                    log::error!("Transaction submission failed: {}", e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            log::error!("Failed to get protocol: {}", e);
            Err(e)
        }
    }
}
```

## Compatibility Notes

- Both synchronous and asynchronous APIs will be maintained in parallel
- Implementations using the synchronous API will continue to work
- For maximum performance, especially in high-concurrency scenarios, we recommend migrating to the async API

## Performance Considerations

- Async API shows the largest performance gains in I/O-bound operations
- Best practices include batching operations and using connection pooling
- For detailed performance analysis, see `docs/layer2/ASYNC_PERFORMANCE_COMPARISON.md`

## Support and Feedback

If you encounter issues migrating to the async API, please file an issue with the tag `async-migration`. The core team is committed to helping you make this transition smoothly.
