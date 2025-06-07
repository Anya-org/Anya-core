//! Error types for the Anya Bitcoin implementation

use thiserror::Error;

/// Result type alias for Anya operations
pub type Result<T> = std::result::Result<T, AnyaError>;
pub type AnyaResult<T> = std::result::Result<T, AnyaError>;

/// Main error type for the Anya Bitcoin implementation
#[derive(Error, Debug)]
pub enum AnyaError {
    #[error("Bitcoin error: {0}")]
    Bitcoin(#[from] bitcoin::consensus::encode::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Peer error: {0}")]
    Peer(#[from] PeerError),
    
    #[error("P2P error: {0}")]
    P2P(#[from] P2PError),
    
    #[error("Futures IO error: {0}")]
    FuturesIo(#[from] futures_io::Error),
    
    #[error("General error: {0}")]
    General(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Hex decoding error: {0}")]
    Hex(#[from] hex::FromHexError),
    
    #[error("Secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),
}

/// P2P network errors
#[derive(Error, Debug)]
pub enum P2PError {
    #[error("Connection failed: {0}")]
    Connection(String),
    
    #[error("Message handling error: {0}")]
    Message(String),
    
    #[error("General P2P error: {0}")]
    General(String),
    
    #[error("Failed to connect to peer: {0}")]
    ConnectionFailed(String),
    
    #[error("Peer {0} disconnected: {1}")]
    PeerDisconnected(String, String),
    
    #[error("Message validation failed: {0}")]
    MessageValidationFailed(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Peer limit reached")]
    PeerLimitReached,
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Bitcoin serialization error: {0}")]
    SerializationError(String),
}

/// Peer management errors
#[derive(Error, Debug)]
pub enum PeerError {
    #[error("Connection timeout")]
    Timeout,
    
    #[error("Invalid peer address: {0}")]
    InvalidAddress(String),
    
    #[error("Peer disconnected: {0}")]
    Disconnected(String),
    
    #[error("Protocol mismatch: {0}")]
    ProtocolMismatch(String),
}


