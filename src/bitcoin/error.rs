// Bitcoin Error Module
// [AIR-3][AIS-3][BPC-3]
//
// Defines error types for Bitcoin module operations

use thiserror::Error;
use std::fmt;

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
    
    /// Invalid private key
    #[error("Invalid private key")]
    InvalidPrivateKey,
    
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
    SPVError(String),
    
    /// Compliance error
    #[error("BPC-{0} requires: {1}")]
    ComplianceError(u8, String),
    
    /// Signing error
    #[error("Signing error")]
    SigningError,
    
    /// Invalid address
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    /// Asset already issued
    #[error("Asset already issued")]
    AssetAlreadyIssued,
    
    /// Other errors
    #[error("{0}")]
    Other(String),
}

/// Result type for Bitcoin operations
pub type BitcoinResult<T> = Result<T, BitcoinError>;

impl From<bitcoin::secp256k1::Error> for BitcoinError {
    fn from(error: bitcoin::secp256k1::Error) -> Self {
        BitcoinError::KeyError(error.to_string())
    }
}

impl From<hex::Error> for BitcoinError {
    fn from(error: hex::Error) -> Self {
        BitcoinError::Other(format!("Hex error: {}", error))
    }
}

impl From<bitcoin_hashes::Error> for BitcoinError {
    fn from(error: bitcoin_hashes::Error) -> Self {
        BitcoinError::Other(format!("Hash error: {}", error))
    }
}

impl From<bitcoin::psbt::Error> for BitcoinError {
    fn from(error: bitcoin::psbt::Error) -> Self {
        BitcoinError::TaprootError(error.to_string())
    }
}

impl From<bitcoin::key::Error> for BitcoinError {
    fn from(error: bitcoin::key::Error) -> Self {
        BitcoinError::KeyError(error.to_string())
    }
}

impl From<bitcoin::ecdsa::Error> for BitcoinError {
    fn from(error: bitcoin::ecdsa::Error) -> Self {
        BitcoinError::Other(format!("Sighash error: {}", error))
    }
}

impl From<std::io::Error> for BitcoinError {
    fn from(error: std::io::Error) -> Self {
        BitcoinError::Other(format!("IO error: {}", error))
    }
}

impl From<anyhow::Error> for BitcoinError {
    fn from(error: anyhow::Error) -> Self {
        BitcoinError::Other(error.to_string())
    }
}

impl From<serde_json::Error> for BitcoinError {
    fn from(error: serde_json::Error) -> Self {
        BitcoinError::Other(format!("JSON error: {}", error))
    }
}

impl From<bitcoin::address::Error> for BitcoinError {
    fn from(error: bitcoin::address::Error) -> Self {
        BitcoinError::InvalidAddress(error.to_string())
    }
}

// Additional helper methods
impl BitcoinError {
    /// Create a wallet error with a message
    pub fn wallet<S: ToString>(msg: S) -> Self {
        BitcoinError::Wallet(msg.to_string())
    }
    
    /// Create a transaction error with a message
    pub fn transaction<S: ToString>(msg: S) -> Self {
        BitcoinError::Transaction(msg.to_string())
    }
    
    /// Create a network error with a message
    pub fn network<S: ToString>(msg: S) -> Self {
        BitcoinError::Network(msg.to_string())
    }
    
    /// Create a protocol error with a message
    pub fn protocol<S: ToString>(msg: S) -> Self {
        BitcoinError::Protocol(msg.to_string())
    }
}

