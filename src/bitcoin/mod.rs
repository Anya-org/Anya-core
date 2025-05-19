// [AIR-3][AIS-3][BPC-3][RES-3] Fixed import to use the correct BitcoinAdapter
// [AIR-3][AIS-3][BPC-3][RES-3] Bitcoin module implementation
// This follows the Bitcoin Development Framework v2.5 standards for hexagonal architecture

// Core modules for Bitcoin functionality
pub mod interface;
pub mod error;
pub mod manager;
pub mod adapters;
pub mod config;
pub mod taproot;
pub mod rust;
pub mod layer2; // Export layer2 module for Layer2Protocol trait

// Re-export key interfaces for easier access
pub use interface::BitcoinInterface;
pub use adapters::BitcoinAdapter;

// [AIR-3][AIS-3][BPC-3][RES-3] Re-export Bitcoin types for convenience
// This follows the Bitcoin Development Framework v2.5 standards for type consistency
pub use bitcoin::{Address, Block, BlockHash, Network, Transaction, Txid};

// Export our manager and config for easier access
pub use manager::BitcoinManager;
pub use config::BitcoinConfig;

#[cfg(test)]
mod tests;
