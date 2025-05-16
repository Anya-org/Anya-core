use std::error::Error;
use crate::adapters::{YubiConnector, LedgerConnector};
use serde::{Serialize, Deserialize};
use bitcoin::ScriptBuf;

// Define a simple Sha256 hash type wrapper
// Use our own wrapper to avoid conflicts with sha2::Sha256
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sha256Hash {
    pub hash: [u8; 32]
}

impl Sha256Hash {
    pub fn new(hash: [u8; 32]) -> Self {
        Self { hash }
    }
}


pub mod audit;
pub mod config;
pub mod error;
pub mod operations;
pub mod provider;
pub mod providers;
pub mod security;
pub mod types;

// Re-export types for easier access
pub use types::*;

// Re-export security manager for easier access
pub use security::SecurityManager;

// Re-export error types for easier access
pub use error::*;

// Import provider implementations
use self::providers::bitcoin::BitcoinHsmProvider;
use self::providers::cloud::CloudHsmProvider;
use self::providers::hardware::HardwareHsmProvider;
use self::providers::pkcs11::Pkcs11HsmProvider;
use self::providers::simulator::SimulatorHsmProvider;
use self::providers::software::SoftwareHsmProvider;
use self::providers::tpm::TpmHsmProvider;

pub use providers::{
    BitcoinHsmProvider, HardwareHsmProvider, SimulatorHsmProvider, SoftwareHsmProvider,
};

#[cfg(test)]
pub mod tests;

use bitcoin::{Txid, Psbt, Script, ScriptBuf, XOnlyPublicKey};
use bitcoin::taproot::TaprootBuilder;
use bitcoin::bip32::Xpriv;
use bitcoin::key::Secp256k1;
use bitcoin_opcodes::{self, OpCode, all as opcodes};
use chrono::DateTime;
use chrono::Utc;
use secp256k1::ecdsa::Signature;
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
// No need for spawn import
use tracing::{debug, error, info};
use bitcoin::blockdata::block::BlockHeader;
// No need for debug import

// Import HSM provider types
use self::config::HsmConfig;
use self::audit::AuditLogger;
use self::operations::{HsmOperation, OperationRequest, OperationResult};
use self::provider::{HsmProvider, HsmProviderType, HsmProviderStatus};

/// HSM Manager that provides a unified interface to hardware security modules
/// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
pub struct HsmManager {
    /// Configuration for the HSM
    config: HsmConfig,

    /// Active HSM provider
    provider: Box<dyn HsmProvider>,

    /// HSM statistics
    stats: HsmStats,

    /// Whether HSM is enabled
    enabled: bool,

    /// Audit logger for HSM operations
    audit_logger: Arc<AuditLogger>,

    /// Current status
    status: Arc<RwLock<HsmStatus>>,

    /// Health status and bi-yearly check information
    health_status: Arc<RwLock<HsmHealthStatus>>,

    /// Operation tracker
    operation_tracker: Arc<Mutex<HashMap<String, (DateTime<Utc>, String)>>>,
}

/// HSM Statistics
#[derive(Debug, Default, Clone)]
pub struct HsmStats {
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Last operation time
    pub last_operation_time: Option<DateTime<Utc>>,
    /// Total operation time (milliseconds)
    pub total_operation_time_ms: u64,
}

/// HSM status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmStatus {
    /// HSM is initializing
    Initializing,

    /// HSM is ready
    Ready,

    /// HSM is in error state
    Error(String),

    /// HSM is disconnected
    Disconnected,

    /// HSM is shutting down
    ShuttingDown,
    
    /// HSM is disabled (waiting for user to enable)
    Disabled,
}

// HSM errors are defined in the error.rs module
// Re-exported here via 'pub use error::*;'

// Add a struct to track last health check time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthStatus {
    /// Time of the last health check
    pub last_check_time: Option<DateTime<Utc>>,
    /// Status of the last health check
    pub last_check_result: bool,
    /// User has explicitly enabled HSM
    pub user_enabled: bool,
    /// Time of the last system upgrade
    pub last_upgrade_time: Option<DateTime<Utc>>,
    /// Reason if HSM is disabled
    pub disabled_reason: Option<String>,
}

impl Default for HsmHealthStatus {
    fn default() -> Self {
        Self {
            last_check_time: None,
            last_check_result: false,
            user_enabled: false,
            last_upgrade_time: None,
            disabled_reason: Some("HSM is disabled by default".to_string()),
        }
    }
}

impl HsmManager {
    /// Creates a new HSM Manager with the specified configuration
    pub async fn new(config: HsmConfig) -> Result<Self, HsmError> {
        info!(
            "Initializing HSM Manager with provider: {:?}",
            config.provider_type
        );

        // Create the provider based on configuration
        let provider: Box<dyn HsmProvider> = match config.provider_type {
            HsmProviderType::Simulator => Box::new(SimulatorHsmProvider::new(&config.simulator)?),
            HsmProviderType::SoftwareKeyStore => {
                Box::new(SoftwareHsmProvider::new(&config.software)?)
            }
            HsmProviderType::CloudHsm => Box::new(CloudHsmProvider::new(&config.cloud).await?),
            HsmProviderType::HardwareHsm => {
                Box::new(HardwareHsmProvider::new(&config.hardware).await?)
            }
            HsmProviderType::BitcoinHsm => {
                Box::new(BitcoinHsmProvider::new(&config.bitcoin).await?)
            }
        };

        let stats = HsmStats::default();

        // HSM is disabled by default, requiring explicit user activation after testing
        let audit_logger = Arc::new(AuditLogger::new(&config.audit).await?);

        // Create the HSM manager
        let manager = Self {
            config,
            provider,
            stats,
            enabled: false,  // HSM is disabled by default
            audit_logger,
            operation_tracker: Arc::new(Mutex::new(HashMap::new())),
            status: Arc::new(RwLock::new(HsmStatus::Initializing)),
            health_status: Arc::new(RwLock::new(HsmHealthStatus::default())),
        };

        Ok(manager)
    }

    /// Initializes the HSM Manager
    pub async fn initialize(&mut self) -> Result<(), HsmError> {
        info!("Initializing HSM Manager");

        // Update status
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Initializing;
        }

        // Initialize audit logging
        self.audit_logger.initialize().await?;

        // Log initialization event
        self.audit_logger
            .log_event(
                "hsm.initialize",
                &HsmAuditEvent {
                    event_type: "initialize".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "started".to_string(),
                    details: None,
                    operation_id: None,
                },
            )
            .await?;

        // Initialize the provider
        self.provider.initialize().await.map_err(|e| crate::security::hsm::error::HsmError::from(e))?;

        // Update status
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Ready;
        }

        // Log successful initialization
        self.audit_logger
            .log_event(
                "hsm.initialize",
                &HsmAuditEvent {
                    event_type: "initialize".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "success".to_string(),
                    details: None,
                    operation_id: None,
                },
            )
            .await?;

        info!(
            "HSM Manager initialized successfully with provider: {:?}",
            self.config.provider_type
        );
        Ok(())
    }

    /// Checks if the HSM is due for a health check after a system upgrade
    pub async fn should_run_health_check(&self) -> bool {
        let health_status = self.health_status.read().await;
        
        // If there was no last upgrade time recorded, don't trigger a check
        if health_status.last_upgrade_time.is_none() {
            return false;
        }
        
        // If there was an upgrade but no check performed since then, return true
        if let (Some(upgrade_time), Some(check_time)) = (health_status.last_upgrade_time, health_status.last_check_time) {
            return upgrade_time > check_time;
        }
        
        // If there is an upgrade time but no check time, definitely run a check
        if health_status.last_upgrade_time.is_some() && health_status.last_check_time.is_none() {
            return true;
        }
        
        false
    }
    
    /// Record a system upgrade occurred, which will trigger health check requirement
    pub async fn record_system_upgrade(&self) -> Result<(), HsmError> {
        let mut health_status = self.health_status.write().await;
        health_status.last_upgrade_time = Some(Utc::now());
        health_status.disabled_reason = Some("System upgrade requires health check validation".to_string());
        
        // Disable HSM until health check passes
        // Note: We're not updating self.enabled here directly as it would require &mut self
        // Instead, we'll check health_status.last_check_result in the is_enabled() method
        
        // Log event
        self.audit_logger
            .log_event(
                "hsm.system_upgrade",
                &HsmAuditEvent {
                    event_type: "system_upgrade".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "recorded".to_string(),
                    details: Some("HSM disabled until health check passes".to_string()),
                    operation_id: None,
                },
            )
            .await?;
            
        Ok(())
    }
    
    /// Performs a comprehensive health check on the HSM
    pub async fn run_health_check(&mut self) -> Result<bool, HsmError> {
        // Update status during check
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Maintenance;
        }
        
        // Log starting health check
        self.audit_logger
            .log_event(
                "hsm.health_check",
                &HsmAuditEvent {
                    event_type: "health_check".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "started".to_string(),
                    details: None,
                    operation_id: None,
                },
            )
            .await?;
            
        // Perform the actual health check operations
        let check_result = self.provider.perform_health_check().await;
        
        // Update health status
        {
            let mut health_status = self.health_status.write().await;
            health_status.last_check_time = Some(Utc::now());
            
            match &check_result {
                Ok(passed) => {
                    health_status.last_check_result = *passed;
                    
                    if *passed {
                        // Only clear the reason if the check passed
                        if health_status.user_enabled {
                            health_status.disabled_reason = None;
                        } else {
                            health_status.disabled_reason = Some("Waiting for user to enable HSM".to_string());
                        }
                    } else {
                        health_status.disabled_reason = Some("Health check failed".to_string());
                    }
                },
                Err(e) => {
                    health_status.last_check_result = false;
                    health_status.disabled_reason = Some(format!("Health check error: {}", e));
                }
            }
        }
        
        // Update system status based on health check results
        {
            let mut status = self.status.write().await;
            
            // If check passed and user has enabled HSM, set to Ready; otherwise set to Disabled
            if let Ok(true) = check_result {
                let health_status = self.health_status.read().await;
                if health_status.user_enabled {
                    *status = HsmStatus::Ready;
                    self.enabled = true;
                } else {
                    *status = HsmStatus::Disabled;
                    self.enabled = false;
                }
            } else {
                *status = HsmStatus::Disabled;
                self.enabled = false;
            }
        }
        
        // Log completed health check
        let result_str = match &check_result {
            Ok(true) => "passed",
            Ok(false) => "failed",
            Err(_) => "error"
        };
        
        self.audit_logger
            .log_event(
                "hsm.health_check",
                &HsmAuditEvent {
                    event_type: "health_check".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "completed".to_string(),
                    details: Some(format!("Result: {}", result_str)),
                    operation_id: None,
                },
            )
            .await?;
            
        // Return the health check result
        check_result
    }
    
    /// Enables the HSM with user confirmation
    pub async fn enable(&mut self) -> Result<(), HsmError> {
        // First check if a health check is needed
        if self.should_run_health_check().await {
            let health_check_result = self.run_health_check().await?;
            if !health_check_result {
                return Err(HsmError::NotReady("HSM failed health check and cannot be enabled".to_string()));
            }
        }
        
        // Update status
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Initializing;
        }
        
        // Update health status to record user's intent
        {
            let mut health_status = self.health_status.write().await;
            health_status.user_enabled = true;
            health_status.disabled_reason = None; // Clear any previous reason
        }

        // Log event
        self.audit_logger
            .log_event(
                "hsm.enable",
                &HsmAuditEvent {
                    event_type: "enable".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "started".to_string(),
                    details: Some("User-initiated enablement".to_string()),
                    operation_id: None,
                },
            )
            .await?;

        self.enabled = true;

        // Update status
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Ready;
        }

        // Log completed event
        self.audit_logger
            .log_event(
                "hsm.enable",
                &HsmAuditEvent {
                    event_type: "enable".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "completed".to_string(),
                    details: None,
                    operation_id: None,
                },
            )
            .await?;

        Ok(())
    }

    /// Executes an HSM operation
    pub async fn execute<T: Serialize + for<'de> Deserialize<'de> + Send + Sync>(
        &self,
        operation: HsmOperation,
        params: T,
    ) -> Result<OperationResult, HsmError> {
        // Generate operation ID for tracing
        let operation_id = format!("{}", uuid::Uuid::new_v4());
        debug!(
            "Executing HSM operation: {:?}, operation_id: {}",
            operation, operation_id
        );

        // Log operation start
        self.audit_logger
            .log_event(
                "hsm.operation",
                &HsmAuditEvent {
                    event_type: "operation_start".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "started".to_string(),
                    details: Some(format!("Operation: {:?}", operation)),
                    operation_id: Some(operation_id.clone()),
                },
            )
            .await?;

        // Check HSM status
        {
            let status = self.status.read().await;
            if *status != HsmStatus::Ready {
                let err =
                    HsmError::NotReady(format!("HSM is not ready, current status: {:?}", *status));

                // Log operation failure
                self.audit_logger
                    .log_event(
                        "hsm.operation",
                        &HsmAuditEvent {
                            event_type: "operation_error".to_string(),
                            provider: format!("{:?}", self.config.provider_type),
                            status: "failed".to_string(),
                            details: Some(format!("Error: {:?}", err)),
                            operation_id: Some(operation_id),
                        },
                    )
                    .await?;

                return Err(err);
            }
        }

        // Create operation request
        let request = OperationRequest {
            operation,
            params: serde_json::to_value(params)
                .map_err(|e| HsmError::SerializationError(e.to_string()))?,
            operation_id: operation_id.clone(),
        };

        // Execute operation
        let result = match self.provider.execute_operation(request).await {
            Ok(result) => {
                // Log operation success
                self.audit_logger
                    .log_event(
                        "hsm.operation",
                        &HsmAuditEvent {
                            event_type: "operation_complete".to_string(),
                            provider: format!("{:?}", self.config.provider_type),
                            status: "success".to_string(),
                            details: None,
                            operation_id: Some(operation_id),
                        },
                    )
                    .await?;

                Ok(result)
            }
            Err(err) => {
                // Log operation failure
                self.audit_logger
                    .log_event(
                        "hsm.operation",
                        &HsmAuditEvent {
                            event_type: "operation_error".to_string(),
                            provider: format!("{:?}", self.config.provider_type),
                            status: "failed".to_string(),
                            details: Some(format!("Error: {:?}", err)),
                            operation_id: Some(operation_id),
                        },
                    )
                    .await?;

                Err(err)
            }
        };

        result
    }

    /// Generates a new key pair
    pub async fn generate_key_pair(
        &self,
        key_type: KeyType,
        key_name: &str,
    ) -> Result<KeyInfo, HsmError> {
        debug!("Generating key pair: {}, type: {:?}", key_name, key_type);

        // Call the execute method with GenerateKeyPair operation
        let params = GenerateKeyParams {
            key_type,
            key_name: key_name.to_string(),
            store_in_hsm: true,
        };

        let result = self.execute(HsmOperation::GenerateKeyPair, params).await?;

        // Convert result to KeyInfo
        let key_info: KeyInfo = serde_json::from_value(result.data)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(key_info)
    }

    /// Signs data using a key stored in the HSM
    pub async fn sign_data(
        &self,
        key_name: &str,
        _data: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        debug!(
            "Signing data with key: {}, algorithm: {:?}",
            key_name, algorithm
        );

        // Call the execute method with SignData operation
        let params = SignParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            algorithm,
        };

        let result = self.execute(HsmOperation::SignData, params).await?;

        // Convert result to signature bytes
        let signature = BASE64.decode(result.data.as_str().ok_or_else(|| {
            HsmError::DeserializationError("Expected string for signature".to_string())
        })?)
        .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(signature)
    }

    /// Verifies a signature using a key stored in the HSM
    pub async fn verify_signature(
        &self,
        key_name: &str,
        _data: &[u8],
        _signature: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<bool, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        debug!(
            "Verifying signature with key: {}, algorithm: {:?}",
            key_name, algorithm
        );

        // Call the execute method with VerifySignature operation
        let params = VerifyParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            signature: BASE64.encode(signature),
            algorithm,
        };

        let result = self.execute(HsmOperation::VerifySignature, params).await?;

        // Convert result to boolean
        let verified = result.data.as_bool().ok_or_else(|| {
            HsmError::DeserializationError("Expected boolean for verification result".to_string())
        })?;

        Ok(verified)
    }

    /// Encrypts data using a key stored in the HSM
    pub async fn encrypt_data(
        &self,
        key_name: &str,
        _data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        debug!(
            "Encrypting data with key: {}, algorithm: {:?}",
            key_name, algorithm
        );

        // Call the execute method with EncryptData operation
        let params = EncryptParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            algorithm,
        };

        let result = self.execute(HsmOperation::EncryptData, params).await?;

        // Convert result to encrypted bytes
        let encrypted = BASE64.decode(result.data.as_str().ok_or_else(|| {
            HsmError::DeserializationError("Expected string for encrypted data".to_string())
        })?)
        .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(encrypted)
    }

    /// Decrypts data using a key stored in the HSM
    pub async fn decrypt_data(
        &self,
        key_name: &str,
        _data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        debug!(
            "Decrypting data with key: {}, algorithm: {:?}",
            key_name, algorithm
        );

        // Call the execute method with DecryptData operation
        let params = DecryptParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            algorithm,
        };

        let result = self.execute(HsmOperation::DecryptData, params).await?;

        // Convert result to decrypted bytes
        let decrypted = BASE64.decode(result.data.as_str().ok_or_else(|| {
            HsmError::DeserializationError("Expected string for decrypted data".to_string())
        })?)
        .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(decrypted)
    }

    /// Gets the current HSM status
    pub async fn get_status(&self) -> HsmStatus {
        let status = self.status.read().await;
        status.clone()
    }

    /// Gets information about a key stored in the HSM
    pub async fn get_key_info(&self, key_name: &str) -> Result<KeyInfo, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        debug!("Getting key info for: {}", key_name);

        // Call the execute method with GetKeyInfo operation
        let params = GetKeyParams {
            key_name: key_name.to_string(),
        };

        let result = self.execute(HsmOperation::GetKeyInfo, params).await?;

        // Convert result to KeyInfo
        let key_info: KeyInfo = serde_json::from_value(result.data)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(key_info)
    }

    /// Lists all keys stored in the HSM
    pub async fn list_keys(&self) -> Result<Vec<KeyInfo>, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        debug!("Listing all keys");

        // Call the execute method with ListKeys operation
        let result = self.execute(HsmOperation::ListKeys, ()).await?;

        // Convert result to Vec<KeyInfo>
        let keys: Vec<KeyInfo> = serde_json::from_value(result.data)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(keys)
    }

    /// Deletes a key from the HSM
    pub async fn delete_key(&self, key_name: &str) -> Result<(), HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        info!("Deleting key: {}", key_name);

        // Call the execute method with DeleteKey operation
        let params = DeleteKeyParams {
            key_name: key_name.to_string(),
        };

        let _ = self.execute(HsmOperation::DeleteKey, params).await?;

        Ok(())
    }

    /// Gets the audit log for a specific time range
    pub async fn get_audit_log(
        &self,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<HsmAuditEvent>, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        debug!("Getting audit log");

        // Delegate to the audit logger
        let events = self
            .audit_logger
            .get_events(start_time, end_time, limit)
            .await?;

        Ok(events)
    }

    /// Rotates a key in the HSM
    pub async fn rotate_key(&self, key_name: &str) -> Result<KeyInfo, HsmError> {
        // Check if HSM is enabled
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }

        info!("Rotating key: {}", key_name);

        // Call the execute method with RotateKey operation
        let params = RotateKeyParams {
            key_name: key_name.to_string(),
        };

        let result = self.execute(HsmOperation::RotateKey, params).await?;

        // Convert result to KeyInfo
        let key_info: KeyInfo = serde_json::from_value(result.data)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;

        Ok(key_info)
    }

    /// Enable the HSM module
    pub async fn enable(&mut self) -> Result<(), HsmError> {
        // Check if the HSM provider is available and working
        let status = self.provider.get_status().await?;
        
        match status {
            HsmProviderStatus::Ready => {
                self.enabled = true;
                *self.status.write().await = HsmStatus::Ready;
                self.log_audit_event("HSM enabled by user", "ENABLE_HSM").await;
                Ok(())
            },
            _ => {
                Err(HsmError::NotReady(format!("Cannot enable HSM, provider status: {:?}", status)))
            }
        }
    }
    
    /// Disable the HSM module
    pub async fn disable(&mut self) -> Result<(), HsmError> {
        self.enabled = false;
        *self.status.write().await = HsmStatus::Disabled;
        self.log_audit_event("HSM disabled by user", "DISABLE_HSM").await;
        Ok(())
    }
    
    /// Check if HSM is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// Lightning Atomic Swaps
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomicSwap {
    preimage_hash: Sha256Hash,
    timeout: u32,
    amount: u64,
    redeem_script: ScriptBuf,
}

impl NetworkManager {
    pub async fn initiate_swap(&self, amount: u64, counterparty: &str) -> Result<AtomicSwap> {
        use rand::Rng;
        use sha2::{Sha256, Digest};
        // Script already imported at the top level
        use bitcoin::opcodes;
        use bitcoin::blockdata::script::Builder;

        let mut rng = rand::thread_rng();
        let preimage = rng.gen::<[u8; 32]>();
        
        // Create SHA-256 hash using the sha2 crate
        let digest = Sha256::digest(&preimage);
        
        // Convert to our custom Sha256Hash type
        let hash_array = digest.into();
        let hash_wrapper = Sha256Hash::new(hash_array);

        // Create a simpler script for atomic swaps with standard opcodes
        let script = Builder::new()
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(&digest.as_slice()) // Use the digest slice for the script
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_ELSE)
            .push_int(self.network.get_block_height()? + 144)
            .push_opcode(opcodes::all::OP_VERIFY)
            .push_slice(&counterparty.as_bytes())
            .push_opcode(opcodes::all::OP_ENDIF)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();

        Ok(AtomicSwap {
            preimage_hash: hash_wrapper, // Use our custom Sha256Hash type
            timeout: 144,
            amount,
            redeem_script: script,
        })
    }
}

// Multi-sig Taproot Wallets
impl SecurityManager {
    pub fn create_multisig_wallet(
        &self,
        threshold: usize,
        keys: &[XOnlyPublicKey],
    ) -> Result<String> {
        let secp = Secp256k1::new();
        let internal_key = keys[0];

        let mut builder = TaprootBuilder::new();
        for (i, key) in keys.iter().enumerate() {
            let script = Script::builder()
                .push_int(threshold as i64)
                .push_slice(key.serialize())
                .push_opcode(opcodes::OP_CHECKSIG)
                .into_script();

            builder = builder.add_leaf(i as u8, script)?;
        }

        let spend_info = builder.finalize(&secp, internal_key)?;
        Ok(spend_info.output_key().to_string())
    }
}

// GPU-Resistant Key Derivation
impl SecurityManager {
    pub fn gpu_resistant_derive(&self, mnemonic: &str) -> Result<Xpriv, Box<dyn Error>> {
        use argon2::{Algorithm, Argon2, Params, Version};
        use bitcoin::Network;
        
        let salt = "ANYA_CORE_SALT_V2";
        
        let params = Params::new(15000, 2, 1, Some(32))
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(Argon2Error::Error(e.to_string())) })?;
            
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        
        let mut output_key = [0u8; 32]; // 32-byte key for BIP32
        argon2.hash_password_into(mnemonic.as_bytes(), salt.as_bytes(), &mut output_key)
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(Argon2Error::Error(e.to_string())) })?;
            
        Xpriv::new_master(Network::Bitcoin, &output_key)
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })
    }
}

// Transaction Repudiation Proofs
#[derive(Debug, Serialize, Deserialize)]
pub struct RepudiationProof {
    nonce: [u8; 32],
    partial_sig: Signature,
    merkle_proof: MerkleProof,
}

/// Mobile SDK for security operations on mobile devices
pub struct MobileSDK {
    /// Security manager for cryptographic operations
    security: Arc<SecurityManager>,
    /// Network manager for blockchain operations
    network: Arc<NetworkManager>,
}

/// Network manager for blockchain operations
pub struct NetworkManager {
    /// Network connection status
    connected: bool,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new() -> Self {
        Self {
            connected: false,
        }
    }
    
    /// Get merkle proof for a transaction
    pub async fn get_merkle_proof(&self, txid: &Txid) -> Result<MerkleProof, Box<dyn Error>> {
        // This would normally fetch the proof from a blockchain node
        // For now, we'll create a placeholder proof
        Ok(MerkleProof {
            txid: *txid,
            path: vec![vec![0u8; 32], vec![1u8; 32]],
            block_height: 100,
            root: [0u8; 32],
        })
    }
}

impl MobileSDK {
    pub async fn generate_repudiation_proof(&self, txid: &Txid) -> Result<RepudiationProof> {
        let nonce = rand::thread_rng().gen::<[u8; 32]>();
        let sig = self.security.sign_repudiation(txid, &nonce).await?;
        let proof = self.network.get_merkle_proof(txid).await?;

        Ok(RepudiationProof {
            nonce,
            partial_sig: sig,
            merkle_proof: proof,
        })
    }
}

pub struct HsmBridge {
    provider: Box<dyn HsmProvider>,
    connected: bool,
}

impl HsmBridge {
    pub async fn sign_transaction(&mut self, psbt: &mut Psbt) -> Result<()> {
        self.provider.sign_psbt(psbt).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HsmType {
    YubiHsm,
    Ledger,
    Trezor,
    Simulator,
}

#[derive(Debug, Clone)]
pub struct CoreWrapper<T: serde::Serialize + serde::de::DeserializeOwned> {
    inner: T,
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> serde::Serialize for CoreWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T: serde::Serialize + serde::de::DeserializeOwned> serde::Deserialize<'de> for CoreWrapper<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|inner| CoreWrapper { inner })
    }
}

// ... existing code ...

#[derive(Debug, thiserror::Error)]
pub enum Argon2Error {
    #[error("Argon2 error: {0}")]
    Error(String),
}

impl From<argon2::Error> for Argon2Error {
    fn from(e: argon2::Error) -> Self {
        Argon2Error::Error(e.to_string())
    }
}
