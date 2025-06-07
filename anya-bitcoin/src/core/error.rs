//! Error types for Bitcoin implementation

use thiserror::Error;
use std::io;

/// Result type alias for Bitcoin implementation
pub type AnyaResult<T> = Result<T, AnyaError>;

/// Error types for Bitcoin implementation
#[derive(Error, Debug)]
pub enum AnyaError {
    /// General error
    #[error("General error: {0}")]
    General(String),
    
    /// Error during Bitcoin network operations
    #[error("Network error: {0}")]
    Network(String),
    
    /// Error during transaction processing
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    /// Error during validation
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Error during consensus operations
    #[error("Consensus error: {0}")]
    Consensus(String),
    
    /// Error during Layer 2 operations
    #[error("Layer 2 error: {0}")]
    Layer2(String),
    
    /// Peer network error
    #[error("Peer error: {0}")]
    Peer(String),
    
    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    /// P2P error
    #[error("P2P error: {0}")]
    P2P(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// Bitcoin core error
    #[error("Bitcoin core error: {0}")]
    Bitcoin(String),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Feature not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl From<secp256k1::Error> for AnyaError {
    fn from(err: secp256k1::Error) -> Self {
        AnyaError::General(err.to_string())
    }
}