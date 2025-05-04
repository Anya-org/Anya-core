use std::error::Error;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::time::Duration;

use crate::security::hsm::provider::HsmProviderType;
use crate::security::hsm::audit::AuditLoggerConfig;

/// Configuration for Hardware Security Module (HSM)
/// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// General HSM settings
    pub general: GeneralConfig,
    
    /// Type of HSM provider to use
    pub provider_type: HsmProviderType,
    
    /// Configuration for SoftHSM provider
    #[serde(default)]
    pub software: SoftHsmConfig,
    
    /// Configuration for CloudHSM provider
    #[serde(default)]
    pub cloud: CloudHsmConfig,
    
    /// Configuration for TPM provider
    #[serde(default)]
    pub tpm: TpmConfig,
    
    /// Configuration for PKCS#11 provider
    #[serde(default)]
    pub pkcs11: Pkcs11Config,
    
    /// Configuration for Simulator provider
    #[serde(default)]
    pub simulator: SimulatorConfig,
    
    /// Configuration for Hardware HSM provider
    #[serde(default)]
    pub hardware: HardwareConfig,
    
    /// Configuration for Bitcoin HSM provider
    #[serde(default)]
    pub bitcoin: BitcoinConfig,
    
    /// Configuration for audit logging
    #[serde(default)]
    pub audit: AuditLoggerConfig,
    
    /// Key management settings
    #[serde(default)]
    pub key_management: KeyManagementConfig,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            provider_type: HsmProviderType::SoftwareKeyStore,
            software: SoftHsmConfig::default(),
            cloud: CloudHsmConfig::default(),
            tpm: TpmConfig::default(),
            pkcs11: Pkcs11Config::default(),
            simulator: SimulatorConfig::default(),
            hardware: HardwareConfig::default(),
            bitcoin: BitcoinConfig::default(),
            audit: AuditLoggerConfig::default(),
            key_management: KeyManagementConfig::default(),
        }
    }
}

impl HsmConfig {
    /// Creates a new configuration for development environment
    pub fn development() -> Self {
        Self {
            general: GeneralConfig {
                enabled: true,
                log_level: LogLevel::Debug,
                operation_timeout: Duration::from_secs(10),
            },
            provider_type: HsmProviderType::SoftHsm,
            softhsm: SoftHsmConfig::default(),
            audit: AuditLoggerConfig {
                enabled: true,
                storage_type: crate::security::hsm::audit::AuditStorageType::File,
                file_path: Some("./logs/hsm_audit_dev.log".to_string()),
                retention_days: 30,
                log_sensitive: true, // Allow sensitive logging in dev
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Creates a new configuration for production environment
    pub fn production() -> Self {
        Self {
            general: GeneralConfig {
                enabled: true,
                log_level: LogLevel::Info,
                operation_timeout: Duration::from_secs(30),
            },
            provider_type: HsmProviderType::Pkcs11, // Use real HSM in production
            pkcs11: Pkcs11Config {
                library_path: "/usr/lib/libpkcs11.so".to_string(),
                slot_id: Some(0),
                user_pin: None, // Should be provided at runtime
                ..Default::default()
            },
            audit: AuditLoggerConfig {
                enabled: true,
                storage_type: crate::security::hsm::audit::AuditStorageType::Database,
                db_connection: Some("postgresql://user:password@localhost:5432/auditdb".to_string()),
                retention_days: 365, // Keep logs for a year
                log_sensitive: false, // No sensitive logging in prod
                ..Default::default()
            },
            key_management: KeyManagementConfig {
                auto_rotation: true,
                rotation_interval: Duration::from_secs(7776000), // 90 days
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

/// General HSM settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Whether HSM is enabled
    pub enabled: bool,
    
    /// Log level
    pub log_level: LogLevel,
    
    /// Timeout for HSM operations
    #[serde(with = "humantime_serde")]
    pub operation_timeout: Duration,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: LogLevel::Info,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Configuration for SoftHSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftHsmConfig {
    /// Path to the token directory
    pub token_dir: String,
    
    /// Slot ID to use
    pub slot_id: u64,
    
    /// User PIN
    pub user_pin: Option<String>,
    
    /// SO PIN (Security Officer)
    pub so_pin: Option<String>,
    
    /// Label for the token
    pub token_label: String,
}

impl Default for SoftHsmConfig {
    fn default() -> Self {
        Self {
            token_dir: "./hsm/tokens".to_string(),
            slot_id: 0,
            user_pin: Some("1234".to_string()), // Default PIN for development
            so_pin: Some("5678".to_string()),   // Default SO PIN for development
            token_label: "anya-hsm-token".to_string(),
        }
    }
}

/// Configuration for CloudHSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudHsmConfig {
    /// Cluster ID for AWS CloudHSM
    pub cluster_id: Option<String>,
    
    /// Endpoint for the HSM cluster
    pub endpoint: Option<String>,
    
    /// User credentials
    pub username: Option<String>,
    
    /// Password
    pub password: Option<String>,
    
    /// Certificate file path
    pub certificate_file: Option<String>,
    
    /// Region for AWS CloudHSM
    pub region: Option<String>,
}

impl Default for CloudHsmConfig {
    fn default() -> Self {
        Self {
            cluster_id: None,
            endpoint: None,
            username: None,
            password: None,
            certificate_file: None,
            region: Some("us-west-2".to_string()),
        }
    }
}

/// Configuration for TPM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpmConfig {
    /// Device path for the TPM
    pub device_path: String,
    
    /// Owner password
    pub owner_password: Option<String>,
    
    /// Storage root key password
    pub srk_password: Option<String>,
    
    /// Use TCG software stack
    pub use_tss: bool,
}

impl Default for TpmConfig {
    fn default() -> Self {
        Self {
            device_path: "/dev/tpm0".to_string(),
            owner_password: None,
            srk_password: None,
            use_tss: true,
        }
    }
}

/// Configuration for PKCS#11 provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pkcs11Config {
    /// Path to the PKCS#11 library
    pub library_path: String,
    
    /// Slot ID to use
    pub slot_id: Option<u64>,
    
    /// Token label
    pub token_label: Option<String>,
    
    /// User PIN
    pub user_pin: Option<String>,
    
    /// Maximum sessions
    pub max_sessions: usize,
    
    /// Read-write sessions
    pub rw_session: bool,
}

impl Default for Pkcs11Config {
    fn default() -> Self {
        Self {
            library_path: "/usr/lib/libpkcs11.so".to_string(),
            slot_id: None,
            token_label: None,
            user_pin: None,
            max_sessions: 10,
            rw_session: true,
        }
    }
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Whether to automatically rotate keys
    pub auto_rotation: bool,
    
    /// Interval for key rotation
    #[serde(with = "humantime_serde")]
    pub rotation_interval: Duration,
    
    /// Key naming pattern
    pub key_naming_pattern: String,
    
    /// Whether to keep old key versions
    pub keep_old_versions: bool,
    
    /// Maximum number of key versions to keep
    pub max_versions: usize,
    
    /// Default key types
    #[serde(default)]
    pub default_key_types: DefaultKeyTypes,
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            auto_rotation: false,
            rotation_interval: Duration::from_secs(7776000), // 90 days
            key_naming_pattern: "{type}-{purpose}-{id}".to_string(),
            keep_old_versions: true,
            max_versions: 3,
            default_key_types: DefaultKeyTypes::default(),
        }
    }
}

/// Default key types for different purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultKeyTypes {
    /// Default key type for signing
    pub signing: String,
    
    /// Default key type for encryption
    pub encryption: String,
    
    /// Default key type for key wrapping
    pub key_wrapping: String,
    
    /// Default key type for authentication
    pub authentication: String,
}

impl Default for DefaultKeyTypes {
    fn default() -> Self {
        Self {
            signing: "ec/p256".to_string(),
            encryption: "rsa/2048".to_string(),
            key_wrapping: "aes/256".to_string(),
            authentication: "ec/p256".to_string(),
        }
    }
} 
