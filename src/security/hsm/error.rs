use thiserror::Error;
use std::fmt;

/// HSM errors
#[derive(Error, Debug)]
pub enum HsmError {
    /// HSM provider error
    #[error("HSM provider error: {0}")]
    ProviderError(String),
    
    /// The provider is not initialized
    #[error("HSM provider not initialized")]
    NotInitialized,
    
    /// The key was not found
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    /// The key usage is not allowed
    #[error("Key usage not allowed")]
    KeyUsageError,
    
    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    
    /// Invalid key type
    #[error("Invalid key type: {0}")]
    InvalidKeyType(String),
    
    /// Unsupported key type
    #[error("Unsupported key type")]
    UnsupportedKeyType,
    
    /// Unsupported algorithm
    #[error("Unsupported algorithm")]
    UnsupportedAlgorithm,
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// HSM is locked
    #[error("HSM is locked")]
    HsmLocked,
    
    /// Operation timeout
    #[error("Operation timeout")]
    Timeout,
    
    /// Feature not implemented
    #[error("Feature not implemented")]
    NotImplemented,
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Audit error
    #[error("Audit error: {0}")]
    AuditError(String),
    
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for HSM operations
pub type HsmResult<T> = Result<T, HsmError>;

/// Audit event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventType {
    /// HSM initialization
    Initialize,
    /// HSM close
    Close,
    /// Key generation
    KeyGeneration,
    /// Key deletion
    KeyDeletion,
    /// Key rotation
    KeyRotation,
    /// Data signing
    Sign,
    /// Signature verification
    Verify,
    /// Data encryption
    Encrypt,
    /// Data decryption
    Decrypt,
    /// Key import
    KeyImport,
    /// Key export
    KeyExport,
    /// Login
    Login,
    /// Logout
    Logout,
    /// Configuration change
    ConfigChange,
    /// Other operation
    Other,
}

impl fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventType::Initialize => write!(f, "Initialize"),
            AuditEventType::Close => write!(f, "Close"),
            AuditEventType::KeyGeneration => write!(f, "KeyGeneration"),
            AuditEventType::KeyDeletion => write!(f, "KeyDeletion"),
            AuditEventType::KeyRotation => write!(f, "KeyRotation"),
            AuditEventType::Sign => write!(f, "Sign"),
            AuditEventType::Verify => write!(f, "Verify"),
            AuditEventType::Encrypt => write!(f, "Encrypt"),
            AuditEventType::Decrypt => write!(f, "Decrypt"),
            AuditEventType::KeyImport => write!(f, "KeyImport"),
            AuditEventType::KeyExport => write!(f, "KeyExport"),
            AuditEventType::Login => write!(f, "Login"),
            AuditEventType::Logout => write!(f, "Logout"),
            AuditEventType::ConfigChange => write!(f, "ConfigChange"),
            AuditEventType::Other => write!(f, "Other"),
        }
    }
}

/// Audit event result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventResult {
    /// Operation succeeded
    Success,
    /// Operation failed
    Failure,
    /// Operation in progress
    InProgress,
}

impl fmt::Display for AuditEventResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventResult::Success => write!(f, "Success"),
            AuditEventResult::Failure => write!(f, "Failure"),
            AuditEventResult::InProgress => write!(f, "InProgress"),
        }
    }
}

/// Audit event severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventSeverity {
    /// Informational event
    Info,
    /// Warning event
    Warning,
    /// Error event
    Error,
    /// Critical event
    Critical,
}

impl fmt::Display for AuditEventSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventSeverity::Info => write!(f, "Info"),
            AuditEventSeverity::Warning => write!(f, "Warning"),
            AuditEventSeverity::Error => write!(f, "Error"),
            AuditEventSeverity::Critical => write!(f, "Critical"),
        }
    }
} 