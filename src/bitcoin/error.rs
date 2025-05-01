use std::error::Error;
use std::fmt;
use thiserror::Error;

/// Bitcoin-related errors
#[derive(Debug, Error)]
pub enum BitcoinError {
    /// Wallet errors
    #[error("Wallet error: {0}")]
    Wallet(String),
    
    /// Transaction errors
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    /// Network errors
    #[error("Network error: {0}")]
    Network(String),
    
    /// Transaction not found
    #[error("Transaction not found")]
    TransactionNotFound,
    
    /// Block not found
    #[error("Block not found")]
    BlockNotFound,
    
    /// Invalid transaction
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    /// Invalid script
    #[error("Invalid script: {0}")]
    InvalidScript(String),
    
    /// Invalid signature
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    /// Invalid sighash
    #[error("Invalid sighash")]
    InvalidSighash,
    
    /// Signature conversion error
    #[error("Signature conversion error")]
    SignatureConversionError,
    
    /// Invalid secret key
    #[error("Invalid secret key")]
    InvalidSecretKey,
    
    /// Taproot error
    #[error("Taproot error: {0}")]
    TaprootError(String),
    
    /// Key error
    #[error("Key error: {0}")]
    KeyError(String),
    
    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    /// SPV error
    #[error("SPV error: {0}")]
    SPV(String),
    
    /// Compliance error
    #[error("BPC-{0} requires: {1}")]
    ComplianceError(u8, String),
    
    /// Other errors
    #[error("{0}")]
    Other(String),
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>;

impl BitcoinError {
    /// Create a new wallet error
    pub fn wallet(msg: &str) -> Self {
        BitcoinError::Wallet(msg.to_string())
    }
    
    /// Create a new transaction error
    pub fn transaction(msg: &str) -> Self {
        BitcoinError::Transaction(msg.to_string())
    }
    
    /// Create a new network error
    pub fn network(msg: &str) -> Self {
        BitcoinError::Network(msg.to_string())
    }
    
    /// Create a new protocol error
    pub fn protocol(msg: &str) -> Self {
        BitcoinError::Protocol(msg.to_string())
    }
    
    /// Create a new SPV error
    pub fn spv(msg: &str) -> Self {
        BitcoinError::SPV(msg.to_string())
    }
    
    /// Create a new compliance error
    pub fn compliance(level: u8, msg: &str) -> Self {
        BitcoinError::ComplianceError(level, msg.to_string())
    }
}

impl From<std::io::Error> for BitcoinError {
    fn from(error: std::io::Error) -> Self {
        BitcoinError::Other(error.to_string())
    }
}

impl From<bitcoin::secp256k1::Error> for BitcoinError {
    fn from(error: bitcoin::secp256k1::Error) -> Self {
        BitcoinError::Other(error.to_string())
    }
}

impl From<bitcoin::consensus::encode::Error> for BitcoinError {
    fn from(error: bitcoin::consensus::encode::Error) -> Self {
        BitcoinError::Other(error.to_string())
    }
}

/// Convert errors from bitcoin_hashes
impl From<bitcoin::hashes::Error> for BitcoinError {
    fn from(error: bitcoin::hashes::Error) -> Self {
        BitcoinError::Other(error.to_string())
    }
}
