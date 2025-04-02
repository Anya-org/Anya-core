// HTTP Transport Error Types [AIS-3][BPC-3]
//
// This module defines error types for the HTTP transport implementation
// following the Bitcoin Development Framework v2.5 requirements

use thiserror::Error;

/// HTTP Transport error types
#[derive(Debug, Error)]
pub enum TransportError {
    /// Socket error
    #[error("Socket error: {0}")]
    SocketError(String),
    
    /// Binding error
    #[error("Binding error: {0}")]
    BindingError(String),
    
    /// Server error
    #[error("Server error: {0}")]
    ServerError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Protocol compliance error
    #[error("Protocol compliance error: {0}")]
    ComplianceError(String),
}

impl From<std::io::Error> for TransportError {
    fn from(err: std::io::Error) -> Self {
        TransportError::SocketError(err.to_string())
    }
}

impl From<hyper::Error> for TransportError {
    fn from(err: hyper::Error) -> Self {
        TransportError::ServerError(err.to_string())
    }
}

impl From<String> for TransportError {
    fn from(err: String) -> Self {
        TransportError::ServerError(err)
    }
}

impl From<&str> for TransportError {
    fn from(err: &str) -> Self {
        TransportError::ServerError(err.to_string())
    }
}
