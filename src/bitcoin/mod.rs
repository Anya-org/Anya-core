pub mod error;
pub mod wallet;
pub mod manager;
pub mod adapters;
pub mod config;
pub mod protocol;
pub mod taproot;

// Re-export Bitcoin types for convenience
pub use bitcoin::{Address, Block, BlockHash, Network, Transaction, Txid};

// Export our manager and config for easier access
pub use manager::BitcoinManager;
pub use config::BitcoinConfig;
