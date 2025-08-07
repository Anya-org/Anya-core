# Blockchain Module

**Compliance Tags**: [AIR-3][AIS-3][AIM-3][AIP-3][BPC-3][RES-3]

## Introduction

The Blockchain module provides a hexagonal architecture implementation for blockchain interaction, offering a clean separation between domain logic and external systems. It enables consistent, protocol-agnostic access to various blockchains while maintaining specific optimizations for each.

## Features

- Protocol-agnostic blockchain interface
- Multi-chain support with unified API
- Block header and transaction validation
- Mempool monitoring and management
- Fee estimation and transaction prioritization
- Chain reorganization detection and handling
- UTXO set management
- P2P network interaction

## Core Components

### BlockchainError

Comprehensive error types for blockchain operations:

```rust
pub enum BlockchainError {
    /// Network error
    NetworkError(String),

    /// Configuration error
    ConfigError(String),

    /// Synchronization error
    SyncError(String),

    /// Block processing error
    BlockProcessingError(String),

    /// Transaction error
    TransactionError(String),

    /// Validation error
    ValidationError(String),

    /// Storage error
    StorageError(String),

    /// RPC error
    RpcError(String),

    /// Serialization error
    SerializationError(String),

    /// Not found error
    NotFoundError(String),

    /// IO error
    IoError(std::io::Error),

    /// Timeout error
    TimeoutError(String),

    /// Internal error
    InternalError(String),
}
```

### BlockchainMetrics

Performance and health metrics for blockchain networks:

```rust
pub struct BlockchainMetrics {
    /// Number of blocks in the chain
    pub block_count: u64,

    /// Number of transactions in the chain
    pub tx_count: u64,

    /// Size of the UTXO set
    pub utxo_set_size: u64,

    /// Difficulty
    pub difficulty: f64,

    /// Estimated hash rate
    pub hash_rate: f64,

    /// Network weight (for PoS chains)
    pub network_weight: Option<f64>,

    /// Block propagation time (ms)
    pub block_propagation_time: u64,

    /// Mempool size (bytes)
    pub mempool_size: u64,

    /// Mempool transaction count
    pub mempool_tx_count: u64,

    /// Fee estimates
    pub fee_estimates: HashMap<u16, u64>,

    /// Timestamp of this measurement
    pub timestamp: u64,
}
```

### BlockchainState

Current state information for a blockchain:

```rust
pub struct BlockchainState {
    /// Chain ID
    pub chain_id: String,

    /// Network name
    pub network: String,

    /// Protocol version
    pub protocol_version: u32,

    /// Best block hash
    pub best_block_hash: String,

    /// Best block height
    pub best_block_height: u64,

    /// Median time past
    pub median_time_past: u64,

    /// Whether the chain is in initial block download
    pub initial_block_download: bool,

    /// Synchronization progress (0.0 to 1.0)
    pub sync_progress: f64,

    /// Chain work
    pub chain_work: String,

    /// Size on disk
    pub size_on_disk: u64,

    /// Number of connections
    pub connection_count: u32,

    /// Verification progress
    pub verification_progress: f64,

    /// Pruned status
    pub pruned: bool,
}
```

## Usage Examples

### Connecting to a Blockchain

```rust
use anya::blockchain::{BlockchainAdapter, BlockchainConfig, BlockchainState};
use std::time::Duration;

async fn connect_to_blockchain() -> Result<BlockchainState, Box<dyn std::error::Error>> {
    // Create configuration
    let config = BlockchainConfig {
        protocol: "bitcoin".to_string(),
        network: "testnet".to_string(),
        rpc_url: "http://localhost:18332".to_string(),
        rpc_user: Some("user".to_string()),
        rpc_password: Some("password".to_string()),
        rpc_timeout: Duration::from_secs(30),
        p2p_enabled: true,
        p2p_port: 18333,
        max_connections: 8,
        cache_size_mb: 450,
        validation_level: "full".to_string(),
        user_agent: "Anya Core/0.1.0".to_string(),
    };

    // Create blockchain adapter
    let adapter = BlockchainAdapter::new(config)?;

    // Initialize and connect
    adapter.initialize().await?;

    // Get blockchain state
    let state = adapter.get_state().await?;
    println!("Connected to {} network", state.network);
    println!("Current height: {}", state.best_block_height);
    println!("Sync progress: {:.2}%", state.sync_progress * 100.0);

    Ok(state)
}
```

### Working with Transactions

```rust
use anya::blockchain::{BlockchainAdapter, Transaction, TransactionOptions};

async fn create_transaction(
    adapter: &BlockchainAdapter,
    recipient_address: &str,
    amount: u64
) -> Result<String, Box<dyn std::error::Error>> {
    // Get current fee estimates
    let fees = adapter.estimate_fee().await?;

    // Create transaction options
    let options = TransactionOptions {
        fee_rate: fees.get(&6).cloned().unwrap_or(5000), // 6 block target
        include_change: true,
        locktime: 0,
        rbf_enabled: true,
    };

    // Create transaction
    let tx = adapter.create_transaction(
        recipient_address,
        amount,
        options
    ).await?;

    // Sign transaction
    let signed_tx = adapter.sign_transaction(&tx).await?;

    // Broadcast transaction
    let txid = adapter.broadcast_transaction(&signed_tx).await?;
    println!("Transaction broadcast: {}", txid);

    Ok(txid)
}
```

### Monitoring Blockchain Events

```rust
use anya::blockchain::{BlockchainAdapter, BlockchainEvent, EventListener};
use std::sync::Arc;
use tokio::sync::mpsc;

async fn monitor_blockchain_events(
    adapter: Arc<BlockchainAdapter>
) -> Result<(), Box<dyn std::error::Error>> {
    // Create event listener
    let (tx, mut rx) = mpsc::channel(100);
    let listener = EventListener::new(tx);

    // Register listener with adapter
    adapter.register_listener(listener).await?;

    // Process events
    println!("Monitoring blockchain events...");
    while let Some(event) = rx.recv().await {
        match event {
            BlockchainEvent::NewBlock(block) => {
                println!("New block: {} (height: {})", block.hash, block.height);
                println!("Transactions: {}", block.tx_count);
            },
            BlockchainEvent::Reorg(old_tip, new_tip, common_ancestor) => {
                println!("Chain reorganization detected!");
                println!("Old tip: {}", old_tip);
                println!("New tip: {}", new_tip);
                println!("Common ancestor: {}", common_ancestor);
            },
            BlockchainEvent::MempoolTransaction(tx) => {
                println!("New mempool transaction: {}", tx.txid);
                println!("Fee rate: {} sat/vB", tx.fee_rate);
            },
            // Handle other events...
            _ => {}
        }
    }

    Ok(())
}
```

## Performance Considerations

The blockchain module is designed with performance in mind, implementing:

1. **Efficient Caching**: Block and transaction caching to minimize network requests
2. **Batch Processing**: Processing multiple operations in batches when possible
3. **Connection Pooling**: Reusing connections to minimize setup/teardown overhead
4. **Asynchronous I/O**: Non-blocking operations to maximize throughput
5. **Optimized Validation**: Tiered validation levels based on security requirements

## Security Considerations

1. **Transaction Validation**: Comprehensive verification of all transactions
2. **Block Validation**: Full validation of block headers, merkle trees, and proof of work
3. **Double-spend Protection**: Detection and prevention of double-spend attempts
4. **Network Segregation**: Proper isolation of mainnet, testnet, and regtest networks
5. **Secure RPC**: Authentication and TLS for remote procedure calls

## Cross-Chain Compatibility

This module supports multiple blockchain protocols through adapters:

- Bitcoin and Bitcoin-derived chains
- Ethereum and EVM-compatible chains
- Other supported chains through the adapter pattern

Each protocol-specific adapter implements the common interfaces while handling the unique requirements of each blockchain.

## For More Information

- See the [Bitcoin Core documentation](https://bitcoin.org/en/developer-documentation)
- Project documentation
