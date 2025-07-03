//! Layer 2 Bitcoin Protocol Implementations [AIR-3][AIS-3][BPC-3][RES-3]
//!
//! This module contains implementations of various Layer 2 protocols
//! that can be used with Bitcoin.

use crate::bitcoin::error::BitcoinResult;
use bitcoin::Transaction;

pub mod rgb;

// Re-export commonly used items
pub use rgb::{
    AssetCreationParams, AssetTransfer, RGBAsset, RGBFactory, RGBManager, TransferStatus,
};

/// Layer2Protocol trait defines the interface for Bitcoin Layer 2 protocols
pub trait Layer2Protocol {
    /// Generate a new address for the Layer 2 protocol
    fn generate_address(&self, address_type: &str) -> BitcoinResult<String>;

    /// Create a transaction specific to the Layer 2 protocol
    fn create_transaction(&self, outputs: Vec<(String, u64)>) -> BitcoinResult<Transaction>;

    /// Verify a merkle proof for the Layer 2 protocol
    fn verify_merkle_proof(&self, tx_hash: &[u8], block_header: &[u8]) -> BitcoinResult<bool>;

    /// Get a transaction by its ID
    fn get_transaction(&self, txid: &str) -> BitcoinResult<Transaction>;

    /// Get a block by its hash
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<u8>>;

    /// Broadcast a transaction to the network
    fn broadcast_transaction(&self, tx: &Transaction) -> BitcoinResult<String>;

    /// Send a transaction to the network
    fn send_transaction(&self, tx: &Transaction) -> BitcoinResult<String>;

    /// Get the current block height
    fn get_block_height(&self) -> BitcoinResult<u64>;

    /// Get the balance for an address
    fn get_balance(&self, address: &str) -> BitcoinResult<u64>;

    /// Estimate the fee for a transaction
    fn estimate_fee(&self) -> BitcoinResult<u64>;
}

/// Registry of Layer 2 protocols
pub struct Layer2Registry(Vec<(String, Box<dyn Layer2Protocol>)>);

impl Default for Layer2Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer2Registry {
    /// Create a new Layer 2 registry
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Register a new Layer 2 protocol
    pub fn register(&mut self, name: String, protocol: Box<dyn Layer2Protocol>) {
        self.0.push((name, protocol));
    }

    /// Get a reference to a registered protocol by name
    pub fn get(&self, name: &str) -> Option<&dyn Layer2Protocol> {
        self.0.iter()
            .find(|(n, _)| n == name)
            .map(|(_, p)| p.as_ref())
    }

    /// List all registered protocol names
    pub fn list_protocols(&self) -> Vec<&str> {
        self.0.iter().map(|(name, _)| name.as_str()).collect()
    }
}
