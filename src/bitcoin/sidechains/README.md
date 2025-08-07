# Bitcoin Sidechains Module

**Compliance Tags**: [AIR-2][AIS-3][AIT-2][AIM-2][AIP-2][BPC-3][PFM-2][RES-3][SCL-2]

## Overview

The `bitcoin/sidechains` module implements unified sidechain management for the Bitcoin ecosystem, allowing seamless integration with various Bitcoin-compatible sidechain networks. Sidechains are separate blockchains that are pegged to Bitcoin, enabling assets to be transferred between chains while maintaining Bitcoin's security guarantees.

Currently, the module supports the following sidechains:

- **RSK (Rootstock)**: A smart contract platform that is merge-mined with Bitcoin
- **Liquid**: A federated sidechain developed by Blockstream for fast settlement and asset issuance
- **Stacks** (in development): A layer that enables smart contracts and decentralized apps using Bitcoin as a security layer

## Core Components

### SidechainType

Enumeration of supported sidechain networks:

```rust
pub enum SidechainType {
    RSK,         // Rootstock - Smart contracts
    Stacks,      // Stacks - Smart contracts and apps
    Liquid,      // Liquid - Asset issuance
    Other(String),  // Other sidechains
}
```

### CrossChainTxStatus

Represents the status of a cross-chain transaction:

```rust
pub enum CrossChainTxStatus {
    /// Transaction is pending on the source chain
    PendingSource,

    /// Transaction is confirmed on the source chain
    ConfirmedSource,

    /// Transaction is pending on the destination chain
    PendingDestination,

    /// Transaction is confirmed on both chains
    Confirmed,

    /// Transaction failed
    Failed(String),
}
```

### CrossChainTx

Representation of cross-chain transactions:

```rust
pub struct CrossChainTx {
    /// Transaction ID
    pub id: String,

    /// Source chain
    pub source_chain: SidechainType,

    /// Destination chain
    pub destination_chain: SidechainType,

    /// Source transaction ID
    pub source_txid: String,

    /// Destination transaction ID (if available)
    pub destination_txid: Option<String>,

    /// Transaction status
    pub status: CrossChainTxStatus,

    /// Transaction amount
    pub amount: String,

    /// Transaction fee
    pub fee: String,

    /// Transaction timestamp
    pub timestamp: u64,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
```

### SidechainStatus

Information about a sidechain's current state:

```rust
pub struct SidechainStatus {
    /// Sidechain type
    pub sidechain_type: SidechainType,

    /// Is the sidechain active
    pub is_active: bool,

    /// Current block height
    pub block_height: u64,

    /// Latest block hash
    pub latest_block_hash: String,

    /// Average block time in seconds
    pub average_block_time: f64,

    /// Chain synchronization percentage
    pub sync_percentage: f64,
}
```

### SidechainManager Trait

The main interface for interacting with sidechains:

```rust
pub trait SidechainManager {
    /// Lists supported sidechains
    fn list_sidechains(&self) -> AnyaResult<Vec<SidechainType>>;

    /// Gets sidechain status
    fn get_sidechain_status(&self, sidechain: &SidechainType) -> AnyaResult<SidechainStatus>;

    /// Lists cross-chain transactions
    fn list_cross_chain_txs(&self) -> AnyaResult<Vec<CrossChainTx>>;

    /// Gets a cross-chain transaction by ID
    fn get_cross_chain_tx(&self, tx_id: &str) -> AnyaResult<Option<CrossChainTx>>;

    /// Gets the status of a cross-chain transaction
    fn get_cross_chain_tx_status(&self, tx_id: &str) -> AnyaResult<CrossChainTxStatus>;
}
```

### SidechainFactory

Factory for creating sidechain managers:

```rust
pub struct SidechainFactory;

impl SidechainFactory {
    /// Creates a new sidechain manager
    pub fn create_manager() -> Box<dyn SidechainManager> {
        Box::new(DefaultSidechainManager::new())
    }
}
```

## Usage Examples

### Creating a Sidechain Manager

```rust
use crate::bitcoin::sidechains::{SidechainFactory, SidechainManager, SidechainType};

fn manage_sidechains() -> AnyaResult<()> {
    // Create a new sidechain manager
    let manager = SidechainFactory::create_manager();

    // List all supported sidechains
    let sidechains = manager.list_sidechains()?;
    println!("Supported sidechains: {:?}", sidechains);

    // Get status of RSK sidechain
    let rsk_status = manager.get_sidechain_status(&SidechainType::RSK)?;
    println!("RSK block height: {}", rsk_status.block_height);

    Ok(())
}
```

### Working with Cross-Chain Transactions

```rust
use crate::bitcoin::sidechains::{SidechainFactory, CrossChainTxStatus};

fn monitor_cross_chain_tx(tx_id: &str) -> AnyaResult<()> {
    let manager = SidechainFactory::create_manager();

    // Get transaction details
    if let Some(tx) = manager.get_cross_chain_tx(tx_id)? {
        println!("Transaction: {} ({})", tx.id, tx.status);

        // Monitor transaction status
        match tx.status {
            CrossChainTxStatus::Confirmed => {
                println!("Transaction confirmed on both chains!");
                println!("Source txid: {}", tx.source_txid);
                println!("Destination txid: {}", tx.destination_txid.unwrap_or_default());
            },
            CrossChainTxStatus::PendingDestination => {
                println!("Transaction confirmed on source chain, pending on destination");
            },
            CrossChainTxStatus::PendingSource => {
                println!("Transaction pending on source chain");
            },
            CrossChainTxStatus::Failed(ref reason) => {
                println!("Transaction failed: {}", reason);
            },
        }
    } else {
        println!("Transaction not found: {}", tx_id);
    }

    Ok(())
}
```

## Sidechain-Specific Implementations

### RSK (Rootstock)

The RSK implementation provides smart contract functionality:

- Smart contract deployment and interaction
- BTC-RBTC two-way peg operations
- RSK node connectivity and management

### Liquid

The Liquid implementation provides confidential transactions and asset issuance:

- Issued asset management
- Confidential transactions
- Federation status monitoring

## Error Handling

The sidechains module uses the standard `AnyaResult` type for error handling. Common errors include:

- Network connectivity issues
- Invalid transaction parameters
- Insufficient peg-in/peg-out funds
- Two-way peg verification failures

Example of proper error handling:

```rust
use crate::bitcoin::sidechains::{SidechainFactory, SidechainType};

fn handle_sidechain_operations() -> AnyaResult<()> {
    let manager = SidechainFactory::create_manager();

    match manager.get_sidechain_status(&SidechainType::Liquid) {
        Ok(status) => {
            if !status.is_active {
                println!("Warning: Liquid sidechain is currently inactive");
                // Implement fallback or retry logic
            }
        },
        Err(e) => {
            eprintln!("Error checking Liquid status: {}", e);
            // Log error, implement recovery strategy
        }
    }

    Ok(())
}
```

## Testing

To test the sidechains implementation, use the provided test utilities:

```bash
# Run all sidechain tests
cargo test --package anya-core --lib bitcoin::sidechains

# Test only RSK implementation
cargo test --package anya-core --lib bitcoin::sidechains::rsk

# Test only Liquid implementation
cargo test --package anya-core --lib bitcoin::sidechains::liquid
```

## Security Considerations

When working with sidechains:

1. Always verify two-way peg proofs before confirming transactions
2. Monitor federation nodes (particularly for Liquid) to ensure network health
3. Use appropriate confirmation thresholds for cross-chain operations
4. Be aware of the security model differences between sidechains and the main Bitcoin blockchain

## Future Developments

Planned additions to the sidechains module include:

- Full Stacks integration
- Support for additional sidechains
- Enhanced monitoring of federation nodes
- Cross-chain atomic swaps

## For more information

- See the comprehensive documentation in the [docs/](../../../docs/) directory
- RSK documentation: [RSK Developers Portal](https://developers.rsk.co/)
- Liquid documentation: [Liquid Network](https://blockstream.com/liquid/)
- Stacks documentation: [Stacks Documentation](https://docs.stacks.co/)
