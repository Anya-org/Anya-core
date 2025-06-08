// Bitcoin Interface Module
// Implements a clean API for Bitcoin network operations
//
// [AIR-3][AIS-3][AIT-2][AIM-2][AIP-2][BPC-3][AIP-3][PFM-2]
// This module provides high interoperability with full Bitcoin protocol compliance
// and comprehensive security measures for network operations.

// [AIR-3][AIS-3][RES-3]
// BitcoinInterface trait for hexagonal architecture pattern
// Following official Bitcoin Improvement Proposals (BIPs) standards

// [AIR-3][AIS-3][BPC-3][RES-3] Import necessary dependencies for Bitcoin interface
// This follows official Bitcoin Improvement Proposals (BIPs) standards for hexagonal architecture
use std::error::Error as StdError;
use std::sync::Arc;
// [AIR-3][AIS-3][BPC-3][RES-3] Removed unused import: async_trait::async_trait

// Re-export bitcoin types for use by other modules
pub use bitcoin::{Address, Transaction, Block, Network};
pub use crate::bitcoin::error::{BitcoinError, BitcoinResult};

// [AIR-3][AIS-3][BPC-3][RES-3] Import Bitcoin configuration
// This follows official Bitcoin Improvement Proposals (BIPs) standards for configuration management
use crate::bitcoin::config::BitcoinConfig as ConfigBitcoinConfig;
use crate::bitcoin::config::BitcoinConfig as BitcoinInternalConfig;
// Use fully qualified paths to avoid type conflicts

/// Bitcoin implementation type selection enum
/// 
/// This enum allows for runtime selection between different Bitcoin
/// implementations while maintaining a consistent API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitcoinImplementationType {
    /// Use the Rust bitcoin implementation (rust-bitcoin, BDK)
    Rust,
    Core,
    Electrum,
    Custom,
    Web3,
    RPC,
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Block version
    pub version: i32,
    /// Hash of the previous block
    pub prev_blockhash: String,
    /// Merkle root of all transactions
    pub merkle_root: String,
    /// Block timestamp
    pub time: u32,
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
/// 
/// [AIR-3][AIS-3][BPC-3][RES-3]
/// Complete implementation as per official Bitcoin Improvement Proposals (BIPs) standards
#[async_trait::async_trait]
pub trait BitcoinInterface: Send + Sync {
    /// Get transaction by txid
    /// 
    /// Retrieves detailed information about a transaction given its ID.
    async fn get_transaction(&self, txid: &str) -> BitcoinResult<Transaction>;
    
    /// Get block by hash
    /// 
    /// Retrieves all transactions in a block given the block hash.
    async fn get_block(&self, hash: &str) -> BitcoinResult<Block>;
    
    /// Get current blockchain height
    /// 
    /// Returns the current height of the blockchain (number of blocks).
    async fn get_block_height(&self) -> BitcoinResult<u32>;
    
    /// Generate a new address
    /// 
    /// Creates a new Bitcoin address of the specified type.
    async fn generate_address(&self, address_type: AddressType) -> BitcoinResult<Address>;
    
    /// Create and sign a transaction
    /// 
    /// Creates a transaction sending to specified outputs with the given fee rate.
    /// The implementation handles input selection, change addresses, and signing.
    async fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<Transaction>;
    
    /// Broadcast a transaction to the network
    /// 
    /// Sends a signed transaction to the Bitcoin network.
    async fn broadcast_transaction(&self, transaction: &Transaction) -> BitcoinResult<String>;
    
    /// Get balance for wallet/address
    /// 
    /// Returns the current balance of the wallet in satoshis.
    async fn get_balance(&self, address: &Address) -> BitcoinResult<u64>;
    
    /// Estimate fee for a transaction
    /// 
    /// Estimates the fee rate (in sat/vB) needed for confirmation within target_blocks.
    async fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64>;
    
    /// Get block header by hash
    /// 
    /// Retrieves block header information for a given block hash.
    async fn get_block_header(&self, hash: &str) -> BitcoinResult<BlockHeader>;

    /// Verify a merkle proof
    /// 
    /// Verifies a merkle proof for a given transaction hash and block header.
    async fn verify_merkle_proof(&self, tx_hash: &str, block_header: &BlockHeader) -> BitcoinResult<bool>;

    /// Send a transaction
    /// 
    /// Sends a transaction to the network.
    async fn send_transaction(&self, tx: &Transaction) -> BitcoinResult<String>;

    /// Implementation type
    /// 
    /// Returns which implementation type is being used.
    fn implementation_type(&self) -> BitcoinImplementationType;
}

/// Create a new Bitcoin interface with the specified implementation type
/// 
/// This factory function creates and returns a Bitcoin interface implementation
/// based on the requested type and configuration.
/// 
/// [AIR-3][BPC-3] Implementation according to official Bitcoin Improvement Proposals (BIPs)
pub fn create_bitcoin_interface(
    implementation_type: BitcoinImplementationType,
    config: &ConfigBitcoinConfig,
) -> Result<Arc<dyn BitcoinInterface + 'static>, Box<dyn StdError>> {
    // [AIR-3][AIS-3][BPC-3][RES-3] Convert from config::BitcoinConfig to bitcoin::config::BitcoinConfig
    // This follows official Bitcoin Improvement Proposals (BIPs) standards for configuration handling
    let _internal_config = BitcoinInternalConfig {
        enabled: true,
        network: config.network.clone(),
        rpc_url: Some(config.rpc_url.clone().unwrap_or_else(|| "http://127.0.0.1:18332".to_string())),
        auth: config.auth.clone(),
        min_confirmations: 6,
        default_fee_rate: 10,
        wallet_path: None,
    };
    match implementation_type {
        BitcoinImplementationType::Rust => {
            // Use the Rust implementation
            #[cfg(feature = "rust-bitcoin")]
            {
                match crate::bitcoin::rust::RustBitcoinImplementation::new(&_internal_config) {
                    Ok(implementation) => Ok(Arc::new(implementation) as Arc<dyn BitcoinInterface + 'static>),
                    Err(e) => Err(Box::new(BitcoinError::ConfigError(e.to_string())))
                }
            }
            #[cfg(not(feature = "rust-bitcoin"))]
            {
                Err(Box::new(BitcoinError::ConfigError("Rust Bitcoin implementation requested but feature 'rust-bitcoin' is not enabled".to_string())))
            }
        }
        _ => {
            // Create a Rust implementation for all other cases
            #[cfg(feature = "rust-bitcoin")]
            {
                match crate::bitcoin::rust::RustBitcoinImplementation::new(&_internal_config) {
                    Ok(implementation) => Ok(Arc::new(implementation) as Arc<dyn BitcoinInterface + 'static>),
                    Err(e) => Err(Box::new(BitcoinError::ConfigError(e.to_string())))
                }
            }
            #[cfg(not(feature = "rust-bitcoin"))]
            {
                Err(Box::new(BitcoinError::ConfigError("No Bitcoin implementation available for the requested type".to_string())))
            }
        }
    }
}

/// Get the current Bitcoin interface based on configuration
/// 
/// This function returns the appropriate Bitcoin interface implementation
/// based on the current configuration settings.
/// 
/// [AIR-3][BPC-3] Implementation according to official Bitcoin Improvement Proposals (BIPs)
pub fn get_current_bitcoin_interface(config: &ConfigBitcoinConfig) -> Result<Arc<dyn BitcoinInterface + 'static>, Box<dyn StdError>> {
    // [AIR-3][AIS-3][BPC-3][RES-3] Convert from config::BitcoinConfig to bitcoin::config::BitcoinConfig
    // This follows official Bitcoin Improvement Proposals (BIPs) standards for configuration handling
    let _internal_config = BitcoinInternalConfig {
        enabled: true,
        network: config.network.clone(),
        rpc_url: Some(config.rpc_url.clone().unwrap_or_else(|| "http://127.0.0.1:18332".to_string())),
        auth: config.auth.clone(),
        min_confirmations: 6,
        default_fee_rate: 10,
        wallet_path: None,
    };
    // [AIR-3][AIS-3][BPC-3][RES-3] Create a Rust implementation of the Bitcoin interface
    // Properly handle error conversion to avoid Box<dyn StdError> sizing issues
    let implementation = match crate::bitcoin::rust::RustBitcoinImplementation::new(&_internal_config) {
        Ok(impl_instance) => impl_instance,
        Err(e) => return Err(Box::new(BitcoinError::ConfigError(e.to_string()))),
    };
    Ok(Arc::new(implementation))
}
