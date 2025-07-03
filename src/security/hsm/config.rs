use chrono::Duration as ChronoDuration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::security::hsm::audit::AuditLoggerConfig;
use crate::security::hsm::provider::HsmProviderType;

/// Configuration for Hardware Security Module (HSM)
/// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Whether HSM functionality is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// General HSM settings
    pub general: GeneralConfig,

    /// Provider type
    pub provider_type: HsmProviderType,

    /// Audit logging enabled
    #[serde(default)]
    pub audit_enabled: bool,

    /// SoftHSM configuration
    #[serde(default)]
    pub software: SoftHsmConfig,

    /// Cloud HSM configuration
    #[serde(default)]
    pub cloud: CloudHsmConfig,

    /// TPM configuration
    #[serde(default)]
    pub tpm: TpmConfig,

    /// PKCS#11 configuration
    #[serde(default)]
    pub pkcs11: Pkcs11Config,

    /// Simulator configuration
    #[serde(default)]
    pub simulator: SimulatorConfig,

    /// Hardware HSM configuration
    #[serde(default)]
    pub hardware: HardwareConfig,

    /// Bitcoin HSM configuration
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
            enabled: true,
            general: GeneralConfig::default(),
            provider_type: HsmProviderType::SoftwareKeyStore,
            audit_enabled: true,
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
            enabled: true,
            general: GeneralConfig {
                enabled: true,
                log_level: LogLevel::Debug,
                operation_timeout: Duration::from_secs(10),
            },
            provider_type: HsmProviderType::SoftwareKeyStore,
            audit_enabled: true,
            software: SoftHsmConfig::default(),
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
            enabled: true,
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
                db_connection: Some(
                    "postgresql://user:password@localhost:5432/auditdb".to_string(),
                ),
                retention_days: 365,  // Keep logs for a year
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

/// Configuration for Software HSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftHsmConfig {
    /// Directory for tokens
    pub token_dir: String,

    /// Maximum sessions
    pub max_sessions: usize,

    /// Encryption key
    pub encryption_key: Option<String>,

    /// Lock timeout in seconds
    pub lock_timeout_seconds: u64,

    /// Always use testnet for Bitcoin operations
    pub use_testnet: bool,
}

impl Default for SoftHsmConfig {
    fn default() -> Self {
        Self {
            token_dir: ".tokens".to_string(),
            max_sessions: 10,
            encryption_key: None,
            lock_timeout_seconds: 300,
            use_testnet: true, // Always use testnet by default
        }
    }
}

/// Configuration for Cloud HSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudHsmConfig {
    /// Cloud provider
    pub provider: CloudProvider,

    /// Region
    pub region: String,

    /// Access key
    pub access_key: Option<String>,

    /// Secret key
    pub secret_key: Option<String>,

    /// Key ID prefix
    pub key_id_prefix: Option<String>,
}

impl Default for CloudHsmConfig {
    fn default() -> Self {
        Self {
            provider: CloudProvider::Aws,
            region: "us-east-1".to_string(),
            access_key: None,
            secret_key: None,
            key_id_prefix: None,
        }
    }
}

/// Cloud providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudProvider {
    /// AWS
    Aws,
    /// GCP
    Gcp,
    /// Azure
    Azure,
}

/// Configuration for TPM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpmConfig {
    /// Device path
    pub device_path: String,

    /// Owner password
    pub owner_password: Option<String>,
}

impl Default for TpmConfig {
    fn default() -> Self {
        Self {
            device_path: "/dev/tpm0".to_string(),
            owner_password: None,
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

/// Configuration for Simulator HSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorConfig {
    /// Path for storing simulator data
    pub storage_path: String,

    /// Simulate latency
    pub simulate_latency: bool,

    /// Latency in milliseconds
    pub latency_ms: u64,

    /// Simulate failures
    pub simulate_failures: bool,

    /// Failure rate (0.0 - 1.0)
    pub failure_rate: f64,

    /// PIN timeout in seconds
    pub pin_timeout_seconds: u64,

    /// Max PIN attempts
    pub max_pin_attempts: u8,

    /// Always use testnet for Bitcoin operations
    #[serde(default = "default_true")]
    pub use_testnet: bool,
}

fn default_true() -> bool {
    true
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            storage_path: ".simulator".to_string(),
            simulate_latency: false,
            latency_ms: 100,
            simulate_failures: false,
            failure_rate: 0.05,
            pin_timeout_seconds: 300,
            max_pin_attempts: 3,
            use_testnet: true, // Always use testnet by default
        }
    }
}

/// Hardware device types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareDeviceType {
    /// YubiHSM
    YubiHsm,
    /// Ledger
    Ledger,
    /// Trezor
    TrezorModel,
    /// Custom
    Custom,
}

/// Configuration for Hardware HSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    /// Hardware device type
    pub device_type: HardwareDeviceType,

    /// Device connection string (e.g., IP, USB path)
    pub connection_string: String,

    /// Authentication key ID
    pub auth_key_id: Option<String>,

    /// Password
    pub password: Option<String>,

    /// Timeout in seconds
    pub timeout_seconds: u64,

    /// Always use testnet for Bitcoin operations
    #[serde(default = "default_true")]
    pub use_testnet: bool,
}

impl Default for HardwareConfig {
    fn default() -> Self {
        Self {
            device_type: HardwareDeviceType::YubiHsm,
            connection_string: "127.0.0.1:12345".to_string(),
            auth_key_id: None,
            password: None,
            timeout_seconds: 30,
            use_testnet: true, // Always use testnet by default
        }
    }
}

/// Bitcoin network type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitcoinNetworkType {
    /// Mainnet (never use in development)
    Mainnet,
    /// Testnet (for testing)
    Testnet,
    /// Regtest (local testing)
    Regtest,
    /// Signet (testing network)
    Signet,
}

impl Default for BitcoinNetworkType {
    fn default() -> Self {
        Self::Testnet // Always default to testnet
    }
}

/// Configuration for Bitcoin HSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    /// Bitcoin network type
    #[serde(default)]
    pub network: BitcoinNetworkType,

    /// Bitcoin RPC URL
    pub rpc_url: Option<String>,

    /// Bitcoin RPC username
    pub rpc_username: Option<String>,

    /// Bitcoin RPC password
    pub rpc_password: Option<String>,

    /// Derivation path template
    pub derivation_path_template: String,

    /// Use segwit addresses
    pub use_segwit: bool,

    /// Use taproot addresses
    pub use_taproot: bool,

    /// Confirm transactions on device
    pub confirm_transactions: bool,

    /// Fee rate in sats/vB
    pub default_fee_rate: u64,
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self {
            network: BitcoinNetworkType::Testnet,
            rpc_url: Some("http://127.0.0.1:18332".to_string()), // Default testnet port
            rpc_username: None,
            rpc_password: None,
            derivation_path_template: "m/84'/1'/0'/0/{index}".to_string(), // Testnet bip84 path
            use_segwit: true,
            use_taproot: true,
            confirm_transactions: true,
            default_fee_rate: 5, // sats/vB
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
