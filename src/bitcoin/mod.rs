// [AIR-3][AIS-3][BPC-3][RES-3] Fixed import to use the correct BitcoinAdapter
// [AIR-3][AIS-3][BPC-3][RES-3] Bitcoin module implementation
// This follows official Bitcoin Improvement Proposals (BIPs) standards for hexagonal architecture

// Core modules for Bitcoin functionality
pub mod adapters;
pub mod bip341;
pub mod compat; // Compatibility module for older import patterns
pub mod config;
pub mod error;
pub mod interface;
pub mod layer2; // Export layer2 module for Layer2Protocol trait
pub mod lightning;
pub mod manager;
pub mod node; // Bitcoin node management
pub mod protocol; // Bitcoin protocol compliance module
pub mod rust;
pub mod taproot;
pub mod validation; // Consolidated validation module
pub mod wallet; // Bitcoin wallet management // Lightning Network implementation

// Re-export key interfaces for easier access
pub use adapters::BitcoinAdapter;
pub use interface::BitcoinInterface;
pub use node::BitcoinNode;
pub use protocol::{BPCLevel, BitcoinProtocol};
pub use wallet::{AddressInfo, BitcoinWallet, WalletConfig};

// [AIR-3][AIS-3][BPC-3][RES-3] Re-export Bitcoin types for convenience
// This follows official Bitcoin Improvement Proposals (BIPs) standards for type consistency
#[cfg(feature = "rust-bitcoin")]
pub use bitcoin::{Address, Block, BlockHash, Network, Transaction, Txid};

// Export our manager and config for easier access
pub use config::BitcoinConfig;
pub use manager::BitcoinManager;

// Re-export compatibility modules for tests
pub use compat::anya_bitcoin;

#[cfg(test)]
mod tests;
