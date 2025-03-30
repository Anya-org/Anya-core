// Bitcoin Adapter - Interface for Bitcoin protocol operations
// Provides standardized adapters for various Bitcoin operations

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Bitcoin protocol version supported by this adapter
pub const BITCOIN_PROTOCOL_VERSION: u32 = 70016;

/// Bitcoin network types supported by this adapter
pub enum Network {
    /// Bitcoin mainnet
    Mainnet,
    /// Bitcoin testnet
    Testnet,
    /// Bitcoin regtest (local testing)
    Regtest,
    /// Bitcoin signet (testing network)
    Signet,
}

/// Bitcoin adapter error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),
}

/// Result type for Bitcoin adapter operations
pub type Result<T> = std::result::Result<T, Error>;

/// Initialize the Bitcoin adapter with default settings
pub fn init() -> Result<()> {
    // For now, just return success
    Ok(())
}
