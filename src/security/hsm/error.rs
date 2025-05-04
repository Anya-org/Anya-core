use std::error::Error;
//! Error types for HSM security module
//!
//! This module provides comprehensive error types for HSM operations,
//! audit logging, and security events.
//! [AIR-3][AIS-3][AIM-3][AIP-3][RES-3]

use std::fmt;
use std::io;
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Main HSM error type
#[derive(Error, Debug)]
pub enum HsmError {
    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// HSM initialization error
    #[error("HSM initialization error: {0}")]
    InitializationError(String),
    
    /// Access denied
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    /// Key not found
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    /// Invalid key type
    #[error("Invalid key type: {0}")]
    InvalidKeyType(String),
    
    /// Key generation error
    #[error("Key generation error: {0}")]
    KeyGenerationError(String),
    
    /// Signing error
    #[error("Signing error: {0}")]
    SigningError(String),
    
    /// Verification error
    #[error("Verification error: {0}")]
    VerificationError(String),
    
    /// Encryption error
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    /// Decryption error
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    
    /// Communication error
    #[error("Communication error: {0}")]
    CommunicationError(String),
    
    /// Timeout error
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    /// Authorization error
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    /// HSM device error
    #[error("HSM device error: {0}")]
    DeviceError(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    /// Audit error
    #[error("Audit error: {0}")]
    AuditError(String),
    
    /// Audit storage error
    #[error("Audit storage error: {0}")]
    AuditStorageError(String),
    
    /// Audit logging error
    #[error("Audit logging error: {0}")]
    AuditLoggingError(String),
    
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
    
    /// Invalid state
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    /// Not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    /// Unsupported feature
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    /// Resource busy
    #[error("Resource busy: {0}")]
    ResourceBusy(String),
    
    /// Resource unavailable
    #[error("Resource unavailable: {0}")]
    ResourceUnavailable(String),
    
    /// Operation cancelled
    #[error("Operation cancelled: {0}")]
    OperationCancelled(String),
    
    /// HSM audit event error
    #[error("HSM audit event error: {0}")]
    HsmAuditEventError(String),
    
    /// Hardware failure
    #[error("Hardware failure: {0}")]
    HardwareFailure(String),
    
    /// Connection failure
    #[error("Connection failure: {0}")]
    ConnectionFailure(String),
    
    /// Unsupported hardware
    #[error("Unsupported hardware: {0}")]
    UnsupportedHardware(String),
    
    /// Hardware requires reset
    #[error("Hardware requires reset: {0}")]
    HardwareReset(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// PIN locked
    #[error("PIN locked: {0}")]
    PinLocked(String),
    
    /// Device locked
    #[error("Device locked: {0}")]
    DeviceLocked(String),
    
    /// Token not found
    #[error("Token not found: {0}")]
    TokenNotFound(String),
    
    /// Slot not found
    #[error("Slot not found: {0}")]
    SlotNotFound(String),
    
    /// Firmware error
    #[error("Firmware error: {0}")]
    FirmwareError(String),
    
    /// Invalid data
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    /// Unsupported key type
    #[error("Unsupported key type")]
    UnsupportedKeyType,
    
    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    /// HSM provider error
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    /// Bitcoin specific error
    #[error("Bitcoin error: {0}")]
    BitcoinError(String),
}

/// HSM audit event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventType {
    /// HSM initialization
    HsmInitialize,
    
    /// HSM key generation
    KeyGeneration,
    
    /// HSM key deletion
    KeyDeletion,
    
    /// HSM key rotation
    KeyRotation,
    
    /// HSM key export
    KeyExport,
    
    /// HSM key import
    KeyImport,
    
    /// HSM signing operation
    Sign,
    
    /// HSM verification operation
    Verify,
    
    /// HSM encryption operation
    Encrypt,
    
    /// HSM decryption operation
    Decrypt,
    
    /// HSM authentication
    Authentication,
    
    /// HSM configuration change
    ConfigChange,
    
    /// HSM policy change
    PolicyChange,
    
    /// HSM firmware update
    FirmwareUpdate,
    
    /// HSM backup
    Backup,
    
    /// HSM restore
    Restore,
    
    /// HSM user management
    UserManagement,
    
    /// HSM role management
    RoleManagement,
    
    /// HSM security alert
    SecurityAlert,
    
    /// HSM audit log access
    AuditLogAccess,
    
    /// HSM operation request
    OperationRequest,
    
    /// HSM operation response
    OperationResponse,
    
    /// Custom event
    Custom(String),
}

impl fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventType::HsmInitialize => write!(f, "hsm.initialize"),
            AuditEventType::KeyGeneration => write!(f, "key.generation"),
            AuditEventType::KeyDeletion => write!(f, "key.deletion"),
            AuditEventType::KeyRotation => write!(f, "key.rotation"),
            AuditEventType::KeyExport => write!(f, "key.export"),
            AuditEventType::KeyImport => write!(f, "key.import"),
            AuditEventType::Sign => write!(f, "operation.sign"),
            AuditEventType::Verify => write!(f, "operation.verify"),
            AuditEventType::Encrypt => write!(f, "operation.encrypt"),
            AuditEventType::Decrypt => write!(f, "operation.decrypt"),
            AuditEventType::Authentication => write!(f, "security.authentication"),
            AuditEventType::ConfigChange => write!(f, "config.change"),
            AuditEventType::PolicyChange => write!(f, "policy.change"),
            AuditEventType::FirmwareUpdate => write!(f, "firmware.update"),
            AuditEventType::Backup => write!(f, "maintenance.backup"),
            AuditEventType::Restore => write!(f, "maintenance.restore"),
            AuditEventType::UserManagement => write!(f, "user.management"),
            AuditEventType::RoleManagement => write!(f, "role.management"),
            AuditEventType::SecurityAlert => write!(f, "security.alert"),
            AuditEventType::AuditLogAccess => write!(f, "audit.access"),
            AuditEventType::OperationRequest => write!(f, "operation.request"),
            AuditEventType::OperationResponse => write!(f, "operation.response"),
            AuditEventType::Custom(name) => write!(f, "custom.{}", name),
        }
    }
}

impl std::str::FromStr for AuditEventType {
    type Err = HsmError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hsm.initialize" => Ok(AuditEventType::HsmInitialize),
            "key.generation" => Ok(AuditEventType::KeyGeneration),
            "key.deletion" => Ok(AuditEventType::KeyDeletion),
            "key.rotation" => Ok(AuditEventType::KeyRotation),
            "key.export" => Ok(AuditEventType::KeyExport),
            "key.import" => Ok(AuditEventType::KeyImport),
            "operation.sign" => Ok(AuditEventType::Sign),
            "operation.verify" => Ok(AuditEventType::Verify),
            "operation.encrypt" => Ok(AuditEventType::Encrypt),
            "operation.decrypt" => Ok(AuditEventType::Decrypt),
            "security.authentication" => Ok(AuditEventType::Authentication),
            "config.change" => Ok(AuditEventType::ConfigChange),
            "policy.change" => Ok(AuditEventType::PolicyChange),
            "firmware.update" => Ok(AuditEventType::FirmwareUpdate),
            "maintenance.backup" => Ok(AuditEventType::Backup),
            "maintenance.restore" => Ok(AuditEventType::Restore),
            "user.management" => Ok(AuditEventType::UserManagement),
            "role.management" => Ok(AuditEventType::RoleManagement),
            "security.alert" => Ok(AuditEventType::SecurityAlert),
            "audit.access" => Ok(AuditEventType::AuditLogAccess),
            "operation.request" => Ok(AuditEventType::OperationRequest),
            "operation.response" => Ok(AuditEventType::OperationResponse),
            _ => {
                if s.starts_with("custom.") {
                    let custom_name = s.strip_prefix("custom.").unwrap_or(s);
                    Ok(AuditEventType::Custom(custom_name.to_string()))
                } else {
                    Err(HsmError::HsmAuditEventError(format!("Unknown audit event type: {}", s)))
                }
            }
        }
    }
}

/// HSM audit event result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventResult {
    /// Operation succeeded
    Success,
    
    /// Operation failed
    Failure,
    
    /// Operation in progress
    InProgress,
    
    /// Operation canceled
    Canceled,
    
    /// Operation rejected
    Rejected,
    
    /// Operation timed out
    Timeout,
}

impl fmt::Display for AuditEventResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventResult::Success => write!(f, "success"),
            AuditEventResult::Failure => write!(f, "failure"),
            AuditEventResult::InProgress => write!(f, "in_progress"),
            AuditEventResult::Canceled => write!(f, "canceled"),
            AuditEventResult::Rejected => write!(f, "rejected"),
            AuditEventResult::Timeout => write!(f, "timeout"),
        }
    }
}

impl std::str::FromStr for AuditEventResult {
    type Err = HsmError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "success" => Ok(AuditEventResult::Success),
            "failure" => Ok(AuditEventResult::Failure),
            "in_progress" => Ok(AuditEventResult::InProgress),
            "canceled" => Ok(AuditEventResult::Canceled),
            "rejected" => Ok(AuditEventResult::Rejected),
            "timeout" => Ok(AuditEventResult::Timeout),
            _ => Err(HsmError::HsmAuditEventError(format!("Unknown audit event result: {}", s))),
        }
    }
}

/// HSM audit event severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventSeverity {
    /// Debug information
    Debug,
    
    /// Informational
    Info,
    
    /// Warning
    Warning,
    
    /// Error
    Error,
    
    /// Critical
    Critical,
    
    /// Alert (requires immediate attention)
    Alert,
    
    /// Emergency (system unusable)
    Emergency,
}

impl fmt::Display for AuditEventSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEventSeverity::Debug => write!(f, "debug"),
            AuditEventSeverity::Info => write!(f, "info"),
            AuditEventSeverity::Warning => write!(f, "warning"),
            AuditEventSeverity::Error => write!(f, "error"),
            AuditEventSeverity::Critical => write!(f, "critical"),
            AuditEventSeverity::Alert => write!(f, "alert"),
            AuditEventSeverity::Emergency => write!(f, "emergency"),
        }
    }
}

impl std::str::FromStr for AuditEventSeverity {
    type Err = HsmError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(AuditEventSeverity::Debug),
            "info" => Ok(AuditEventSeverity::Info),
            "warning" => Ok(AuditEventSeverity::Warning),
            "error" => Ok(AuditEventSeverity::Error),
            "critical" => Ok(AuditEventSeverity::Critical),
            "alert" => Ok(AuditEventSeverity::Alert),
            "emergency" => Ok(AuditEventSeverity::Emergency),
            _ => Err(HsmError::HsmAuditEventError(format!("Unknown audit event severity: {}", s))),
        }
    }
}

/// HSM audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAuditEvent {
    /// Event type
    pub event_type: AuditEventType,
    
    /// Event result
    pub result: AuditEventResult,
    
    /// Event severity
    pub severity: AuditEventSeverity,
    
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Event ID
    pub id: String,
    
    /// User ID (if applicable)
    pub user_id: Option<String>,
    
    /// Key ID (if applicable)
    pub key_id: Option<String>,
    
    /// Operation parameters (if applicable)
    pub parameters: Option<serde_json::Value>,
    
    /// Error message (if failure)
    pub error: Option<String>,
    
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

impl HsmAuditEvent {
    /// Create a new audit event
    pub fn new(
        event_type: AuditEventType,
        result: AuditEventResult,
        severity: AuditEventSeverity,
    ) -> Self {
        Self {
            event_type,
            result,
            severity,
            timestamp: chrono::Utc::now(),
            id: uuid::Uuid::new_v4().to_string(),
            user_id: None,
            key_id: None,
            parameters: None,
            error: None,
            metadata: None,
        }
    }
    
    /// Create a success event
    pub fn success(event_type: AuditEventType) -> Self {
        Self::new(event_type, AuditEventResult::Success, AuditEventSeverity::Info)
    }
    
    /// Create a failure event
    pub fn failure(event_type: AuditEventType, error: impl Into<String>) -> Self {
        let mut event = Self::new(event_type, AuditEventResult::Failure, AuditEventSeverity::Error);
        event.error = Some(error.into());
        event
    }
    
    /// Set user ID
    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
    
    /// Set key ID
    pub fn with_key(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }
    
    /// Set parameters
    pub fn with_parameters<T: Serialize>(mut self, parameters: &T) -> Result<Self, HsmError> {
        self.parameters = Some(serde_json::to_value(parameters)
            .map_err(|e| HsmError::SerializationError(format!("Failed to serialize parameters: {}", e)))?);
        Ok(self)
    }
    
    /// Set metadata
    pub fn with_metadata<T: Serialize>(mut self, metadata: &T) -> Result<Self, HsmError> {
        self.metadata = Some(serde_json::to_value(metadata)
            .map_err(|e| HsmError::SerializationError(format!("Failed to serialize metadata: {}", e)))?);
        Ok(self)
    }
}

/// HSM key type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmKeyType {
    /// AES key
    Aes,
    
    /// ECDSA key
    Ecdsa,
    
    /// EdDSA key
    EdDsa,
    
    /// RSA key
    Rsa,
    
    /// HMAC key
    Hmac,
    
    /// Generic symmetric key
    Symmetric,
    
    /// Generic asymmetric key
    Asymmetric,
    
    /// Custom key type
    Custom(u8),
}

impl fmt::Display for HsmKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HsmKeyType::Aes => write!(f, "aes"),
            HsmKeyType::Ecdsa => write!(f, "ecdsa"),
            HsmKeyType::EdDsa => write!(f, "eddsa"),
            HsmKeyType::Rsa => write!(f, "rsa"),
            HsmKeyType::Hmac => write!(f, "hmac"),
            HsmKeyType::Symmetric => write!(f, "symmetric"),
            HsmKeyType::Asymmetric => write!(f, "asymmetric"),
            HsmKeyType::Custom(id) => write!(f, "custom-{}", id),
        }
    }
}

/// HSM operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmOperationType {
    /// Key generation
    GenerateKey,
    
    /// Signing
    Sign,
    
    /// Verification
    Verify,
    
    /// Encryption
    Encrypt,
    
    /// Decryption
    Decrypt,
    
    /// Key derivation
    DeriveKey,
    
    /// Key wrapping (encryption)
    WrapKey,
    
    /// Key unwrapping (decryption)
    UnwrapKey,
    
    /// Key export
    ExportKey,
    
    /// Key import
    ImportKey,
    
    /// Key deletion
    DeleteKey,
    
    /// List keys
    ListKeys,
    
    /// Custom operation
    Custom(u8),
}

impl fmt::Display for HsmOperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HsmOperationType::GenerateKey => write!(f, "generate_key"),
            HsmOperationType::Sign => write!(f, "sign"),
            HsmOperationType::Verify => write!(f, "verify"),
            HsmOperationType::Encrypt => write!(f, "encrypt"),
            HsmOperationType::Decrypt => write!(f, "decrypt"),
            HsmOperationType::DeriveKey => write!(f, "derive_key"),
            HsmOperationType::WrapKey => write!(f, "wrap_key"),
            HsmOperationType::UnwrapKey => write!(f, "unwrap_key"),
            HsmOperationType::ExportKey => write!(f, "export_key"),
            HsmOperationType::ImportKey => write!(f, "import_key"),
            HsmOperationType::DeleteKey => write!(f, "delete_key"),
            HsmOperationType::ListKeys => write!(f, "list_keys"),
            HsmOperationType::Custom(id) => write!(f, "custom-{}", id),
        }
    }
} 
