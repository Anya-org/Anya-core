# Layer2 Async API Documentation

**Date: June 22, 2025**

> **Status (June 2025):**
> The Layer2 async API and related modules are under active development. The implementation is not yet production-ready. Some features described below may not work as intended due to unresolved build and logic errors, especially in `Layer2Manager` and HSM/security modules. See [ROADMAP.md](/docs/ROADMAP.md) for up-to-date status and actionable items.

> **Known Issues:**
>
> - Critical errors remain in Layer2Manager and HSM/security modules; many async methods may fail or be incomplete.
> - Not all tests pass; do not rely on this API for production use.
> - Documentation and code are being actively aligned—expect breaking changes.
> - For details, see the main [ROADMAP.md](/docs/ROADMAP.md).

This document provides a comprehensive reference for the asynchronous API of the Layer2 modules in Anya-core.

## Layer2Manager Async API

The `Layer2Manager` is the central interface for working with Layer2 protocols. It provides methods to initialize protocols and access their functionality.

### Initialization

```rust
/// Initialize all Layer2 protocols asynchronously
pub async fn initialize_all_async(&self) -> Result<(), Layer2Error>
```

Example usage:

```rust
let layer2_manager = Layer2Manager::new();
layer2_manager.initialize_all_async().await?;
```

### Protocol Access

```rust
/// Get a Layer2 protocol implementation by type asynchronously
pub async fn get_protocol_async(&self, protocol_type: Layer2ProtocolType) 
    -> Result<Box<dyn Layer2ProtocolAsync + Send + Sync>, Layer2Error>
```

Example usage:

```rust
let bob_protocol = layer2_manager.get_protocol_async(Layer2ProtocolType::Bob).await?;
```

### Cross-Layer Operations

```rust
/// Perform cross-layer asset transfer asynchronously
pub async fn cross_layer_transfer_async(
    &self,
    asset_id: &AssetId,
    amount: u64,
    source_protocol: Layer2ProtocolType,
    destination_protocol: Layer2ProtocolType,
) -> Result<TransferProof, Layer2Error>

/// Verify cross-layer transfer proof asynchronously
pub async fn verify_cross_layer_proof_async(
    &self,
    proof: &TransferProof,
) -> Result<bool, Layer2Error>
```

Example usage:

```rust
let proof = layer2_manager.cross_layer_transfer_async(
    &asset_id,
    100_000,
    Layer2ProtocolType::Bob,
    Layer2ProtocolType::Lightning,
).await?;

let is_valid = layer2_manager.verify_cross_layer_proof_async(&proof).await?;
```

## Layer2ProtocolAsync Trait

The `Layer2ProtocolAsync` trait defines the interface for async operations on Layer2 protocols.

```rust
#[async_trait]
pub trait Layer2ProtocolAsync {
    /// Initialize the protocol asynchronously
    async fn initialize_async(&self) -> Result<(), Layer2Error>;
    
    /// Submit transaction to the Layer2 protocol asynchronously
    async fn submit_transaction_async(&self, transaction: &Transaction) -> Result<TxStatus, Layer2Error>;
    
    /// Get transaction status asynchronously
    async fn get_transaction_status_async(&self, tx_id: &TxId) -> Result<TxStatus, Layer2Error>;
    
    /// Transfer asset on the Layer2 protocol asynchronously
    async fn transfer_asset_async(
        &self,
        asset_id: &AssetId,
        amount: u64,
        recipient: &Address,
    ) -> Result<TxId, Layer2Error>;
    
    /// Verify proof on the Layer2 protocol asynchronously
    async fn verify_proof_async(&self, proof: &Proof) -> Result<bool, Layer2Error>;
}
```

## Protocol-Specific Implementations

Each Layer2 protocol implements the `Layer2ProtocolAsync` trait with protocol-specific functionality.

### BobClient

```rust
#[async_trait]
impl Layer2ProtocolAsync for BobClient {
    async fn initialize_async(&self) -> Result<(), Layer2Error> {
        // Implementation details
    }
    
    async fn submit_transaction_async(&self, transaction: &Transaction) -> Result<TxStatus, Layer2Error> {
        // Implementation details
    }
    
    // Other methods implemented...
}
```

### LightningNetwork

```rust
#[async_trait]
impl Layer2ProtocolAsync for LightningNetwork {
    async fn initialize_async(&self) -> Result<(), Layer2Error> {
        // Implementation details
    }
    
    async fn transfer_asset_async(
        &self,
        asset_id: &AssetId,
        amount: u64,
        recipient: &Address,
    ) -> Result<TxId, Layer2Error> {
        // LN-specific implementation for transferring assets
    }
    
    // Other methods implemented...
}
```

## Error Handling

All async methods return a `Result` with `Layer2Error` for error handling. Error types specific to async operations include:

```rust
pub enum Layer2Error {
    // Existing error types...
    
    /// Error indicating an async operation timeout
    AsyncTimeout(String),
    
    /// Error indicating an async runtime error
    AsyncRuntimeError(String),
    
    /// Error indicating a task cancellation
    TaskCancelled(String),
}
```

## Advanced Usage Patterns

### Concurrent Operations

```rust
use futures::future::join_all;

async fn process_transactions(txs: Vec<Transaction>) -> Vec<Result<TxStatus, Layer2Error>> {
    let layer2_manager = Layer2Manager::new();
    layer2_manager.initialize_all_async().await?;
    
    let protocol = layer2_manager.get_protocol_async(Layer2ProtocolType::Bob).await?;
    
    let futures = txs.iter()
        .map(|tx| protocol.submit_transaction_async(tx))
        .collect::<Vec<_>>();
    
    join_all(futures).await
}
```

### Timeout Handling

```rust
use tokio::time::{timeout, Duration};

async fn submit_with_timeout(
    protocol: &dyn Layer2ProtocolAsync,
    tx: &Transaction,
    timeout_duration: Duration,
) -> Result<TxStatus, Layer2Error> {
    timeout(timeout_duration, protocol.submit_transaction_async(tx))
        .await
        .map_err(|_| Layer2Error::AsyncTimeout("Transaction submission timed out".into()))?
}
```

### Cancellation Handling

```rust
use tokio::select;
use tokio::sync::oneshot;

async fn cancellable_transfer(
    protocol: &dyn Layer2ProtocolAsync,
    asset_id: &AssetId,
    amount: u64,
    recipient: &Address,
    cancel_rx: oneshot::Receiver<()>,
) -> Result<TxId, Layer2Error> {
    select! {
        result = protocol.transfer_asset_async(asset_id, amount, recipient) => result,
        _ = cancel_rx => Err(Layer2Error::TaskCancelled("Transfer was cancelled".into())),
    }
}
```

## Configuration

Async operations can be configured through the Layer2Config struct:

```rust
pub struct Layer2Config {
    // Existing fields...
    
    /// Maximum number of concurrent operations
    pub max_concurrency: usize,
    
    /// Default timeout for async operations in milliseconds
    pub default_timeout_ms: u64,
    
    /// Connection pool settings
    pub connection_pool: ConnectionPoolConfig,
}
```

## Testing

Use the testing utilities provided for testing async implementations:

```rust
#[tokio::test]
async fn test_bob_transfer() {
    let mock_protocol = MockLayer2Protocol::new();
    // Set up expectations
    
    let result = mock_protocol.transfer_asset_async(&asset_id, 100, &address).await;
    // Assert expectations
}
```

## Compatibility

The async API is designed to work alongside the existing synchronous API. For code that cannot be migrated to async, synchronous wrappers are provided:

```rust
// Using async code in sync context (blocks the current thread)
fn submit_transaction_sync(&self, tx: &Transaction) -> Result<TxStatus, Layer2Error> {
    tokio::runtime::Handle::current().block_on(async {
        self.submit_transaction_async(tx).await
    })
}
```

For detailed migration guidance, please see the [Sync to Async Migration Guide](/docs/layer2/SYNC_TO_ASYNC_MIGRATION_GUIDE.md).
