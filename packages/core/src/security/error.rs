//! Security-related error types
//! This module defines error types for security operations

use thiserror::Error;

/// Errors related to security operations
#[derive(Debug, Error)]
pub enum SecurityError {
    /// Error during HSM operations
    #[error("HSM error: {0}")]
    HsmError(String),
    
    /// Error during key management
    #[error("Key management error: {0}")]
    KeyError(String),
    
    /// Error during secure storage operations
    #[error("Secure storage error: {0}")]
    StorageError(String),
    
    /// Error during credential management
    #[error("Credential error: {0}")]
    CredentialError(String),
    
    /// General security error
    #[error("Security error: {0}")]
    GeneralError(String),
}
