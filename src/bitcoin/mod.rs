pub mod error;
pub mod wallet;

// Re-export Bitcoin types for convenience
pub use bitcoin::{Address, Block, BlockHash, Network, Transaction, Txid};
