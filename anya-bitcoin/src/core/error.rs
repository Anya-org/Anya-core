//! Error types for Bitcoin implementation

use thiserror::Error;
use std::io;

/// Result type alias for Bitcoin implementation
pub type AnyaResult<T> = Result<T, AnyaError>;

/// Error types for Bitcoin implementation
#[derive(Error, Debug)]
pub enum AnyaError {
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