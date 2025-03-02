// Migrated from OPSource to anya-core
// This file was automatically migrated as part of the Rust-only implementation
// Original file: C:\Users\bmokoka\Downloads\OPSource\src\bitcoin\interface.rs
// Bitcoin Interface Layer
// Provides a unified interface for Bitcoin operations with Rust implementation.
//
// This module implements the "ports and adapters" pattern from hexagonal architecture,
// allowing different implementations to be swapped at runtime while maintaining
// a consistent API for the rest of the application. This facilitates:
//
// 1. Testing core business logic against multiple implementations
// 2. Runtime selection of implementation based on features or configuration
// 3. Clear separation of interface (this file) from implementation details

use std::sync::Arc;

/// Bitcoin implementation type selection enum
/// 
/// This enum allows for runtime selection between different Bitcoin
/// implementations while maintaining a consistent API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitcoinImplementationType {
    /// Use the Rust bitcoin implementation (rust-bitcoin, BDK)
    Rust,
}

/// Common error type for Bitcoin operations
/// 
/// Provides a unified error handling approach across implementations
/// while maintaining meaningful context about the error source.
#[derive(Debug, thiserror::Error)]
pub enum BitcoinError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Wallet error: {0}")]
    WalletError(String),
    
    #[error("Block error: {0}")]
    BlockError(String),
    
    #[error("Implementation error: {0}")]
    ImplementationError(String),
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>;

/// Generic Bitcoin address type that works across implementations
/// 
/// This abstraction allows us to represent Bitcoin addresses
/// consistently regardless of the underlying implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAddress {
    /// The actual Bitcoin address string (e.g., "bc1q...")
    pub address: String,
    /// The type of address (P2PKH, P2WPKH, etc.)
    pub address_type: AddressType,
}

/// Address types supported by both implementations
/// 
/// These represent all the major Bitcoin address types supported
/// across our implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// Legacy addresses (1...)
    P2PKH,
    /// Pay to Script Hash addresses (3...)
    P2SH,
    /// SegWit v0 addresses (bc1q...)
    P2WPKH,
    /// SegWit v0 script addresses
    P2WSH,
    /// Taproot addresses (SegWit v1, bc1p...)
    P2TR,
}

/// Transaction representation that works across implementations
/// 
/// This provides a common structure for representing Bitcoin transactions
/// regardless of the underlying implementation details.
#[derive(Debug, Clone)]
pub struct BitcoinTransaction {
    /// Transaction ID (hash)
    pub txid: String,
    /// Transaction version number
    pub version: u32,
    /// Transaction inputs (sources of funds)
    pub inputs: Vec<TransactionInput>,
    /// Transaction outputs (destinations of funds)
    pub outputs: Vec<TransactionOutput>,
    /// Transaction locktime
    pub locktime: u32,
    /// Transaction size in bytes
    pub size: usize,
    /// Transaction weight for fee calculation
    pub weight: usize,
    /// Optional transaction fee in satoshis
    pub fee: Option<u64>,
}

/// Transaction input data
/// 
/// Represents a source of funds in a Bitcoin transaction
#[derive(Debug, Clone)]
pub struct TransactionInput {
    /// Reference to the transaction containing the output being spent
    pub txid: String,
    /// The output index in the referenced transaction
    pub vout: u32,
    /// Script that satisfies the spending conditions
    pub script_sig: Vec<u8>,
    /// Sequence number (used for replace-by-fee, timelocks)
    pub sequence: u32,
    /// Witness data for SegWit transactions
    pub witness: Option<Vec<Vec<u8>>>,
}

/// Transaction output data
/// 
/// Represents a destination of funds in a Bitcoin transaction
#[derive(Debug, Clone)]
pub struct TransactionOutput {
    /// Amount in satoshis
    pub value: u64,
    /// Script defining spending conditions
    pub script_pubkey: Vec<u8>,
    /// Optional human-readable address
    pub address: Option<String>,
}

/// Block header information
/// 
/// Contains the core data from a Bitcoin block header
#[derive(Debug, Clone)]
pub struct BlockHeader {
    /// Block hash
    pub hash: String,
    /// Block version
    pub version: i32,
    /// Hash of the previous block
    pub prev_hash: String,
    /// Merkle root of all transactions
    pub merkle_root: String,
    /// Block timestamp
    pub timestamp: u32,
    /// Difficulty target in compact format
    pub bits: u32,
    /// Nonce value for proof of work
    pub nonce: u32,
}

/// Common interface for Bitcoin operations
/// 
/// This trait defines the contract that all Bitcoin implementations must fulfill.
/// It follows the "port" concept from hexagonal architecture, allowing different
/// adapters (implementations) to be plugged in while maintaining a consistent API.
pub trait BitcoinInterface: Send + Sync {
    /// Get transaction by txid
    /// 
    /// Retrieves detailed information about a transaction given its ID.
    fn get_transaction(&self, txid: &str) -> BitcoinResult<BitcoinTransaction>;
    
    /// Get block by hash
    /// 
    /// Retrieves all transactions in a block given the block hash.
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<BitcoinTransaction>>;
    
    /// Get current blockchain height
    /// 
    /// Returns the current height of the blockchain (number of blocks).
    fn get_block_height(&self) -> BitcoinResult<u32>;
    
    /// Generate a new address
    /// 
    /// Creates a new Bitcoin address of the specified type.
    fn generate_address(&self, address_type: AddressType) -> BitcoinResult<BitcoinAddress>;
    
    /// Create and sign a transaction
    /// 
    /// Creates a transaction sending to specified outputs with the given fee rate.
    /// The implementation handles input selection, change addresses, and signing.
    fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<BitcoinTransaction>;
    
    /// Broadcast a transaction to the network
    /// 
    /// Sends a signed transaction to the Bitcoin network.
    fn broadcast_transaction(&self, transaction: &BitcoinTransaction) -> BitcoinResult<String>;
    
    /// Get balance for wallet/address
    /// 
    /// Returns the current balance of the wallet in satoshis.
    fn get_balance(&self) -> BitcoinResult<u64>;
    
    /// Estimate fee for a transaction
    /// 
    /// Estimates the fee rate (in sat/vB) needed for confirmation within target_blocks.
    fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64>;
    
    /// Implementation type
    /// 
    /// Returns which implementation type is being used.
    fn implementation_type(&self) -> BitcoinImplementationType;
}

/// Create a new Bitcoin interface with the specified implementation type
/// 
/// This factory function creates and returns a Bitcoin interface implementation
/// based on the requested type and configuration.
pub fn create_bitcoin_interface(
    implementation_type: BitcoinImplementationType,
    config: &crate::config::Config,
) -> Arc<dyn BitcoinInterface> {
    match implementation_type {
        BitcoinImplementationType::Rust => {
            let implementation = crate::bitcoin::rust::RustBitcoinImplementation::new(config);
            Arc::new(implementation)
        }
    }
}

/// Get the current Bitcoin interface based on configuration
/// 
/// This function returns the appropriate Bitcoin interface implementation
/// based on the current configuration settings.
pub fn get_current_bitcoin_interface(config: &crate::config::Config) -> Arc<dyn BitcoinInterface> {
    // Always use Rust implementation
    create_bitcoin_interface(BitcoinImplementationType::Rust, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interface_creation() {
        let mut config = crate::config::Config::default();
        
        // Test Rust implementation
        let rust_impl = get_current_bitcoin_interface(&config);
        assert_eq!(rust_impl.implementation_type(), BitcoinImplementationType::Rust);
    }
} 
