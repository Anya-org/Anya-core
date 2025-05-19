pub mod interface;
pub use interface::{BitcoinInterface, BitcoinAdapter};
pub mod error;

pub mod manager;
pub mod adapters;
pub mod config;
pub mod taproot;
pub mod rust;

// Re-export Bitcoin types for convenience
pub use bitcoin::{Address, Block, BlockHash, Network, Transaction, Txid};

// Export our manager and config for easier access
pub use manager::BitcoinManager;
pub use config::BitcoinConfig;

#[cfg(test)]
mod tests;
