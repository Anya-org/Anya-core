//! Hardware Security Module (HSM) Implementation
//!
//! This module provides a unified interface for hardware security operations
//! with a focus on open-source solutions that align with Bitcoin's philosophy.

use crate::security::hsm::audit::AuditLogger;
use crate::security::hsm::providers::hardware::HardwareHsmProvider;
use crate::security::hsm::providers::simulator::SimulatorHsmProvider;
use crate::security::hsm::types::{
    DecryptParams, DeleteKeyParams, EncryptParams, EncryptionAlgorithm, GenerateKeyParams,
    GetKeyParams, HsmRequest, KeyInfo, KeyType, MerkleProof, RotateKeyParams, SignParams,
    SignatureAlgorithm, VerifyParams,
};
use bitcoin::ScriptBuf;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
<<<<<<< HEAD
use crate::security::hsm::audit::AuditLogger;
=======
>>>>>>> feature/git-workflows-consolidation-evidence-based

pub use providers::{
    ledger::LedgerHsmProvider, pkcs11::Pkcs11HsmProvider, software::SoftwareHsmProvider,
    tpm::TpmHsmProvider,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sha256Hash {
    pub hash: [u8; 32],
}

impl Sha256Hash {
    pub fn new(hash: [u8; 32]) -> Self {
        Self { hash }
    }
}

pub mod audit;
pub mod compat;
pub mod config;
pub mod error;
mod error_impls;
pub mod operations;
pub mod provider;
pub mod providers;
pub mod security;
pub mod types;

pub use error::*;
pub use provider::HsmProviderStatus;
pub use providers::bitcoin::BitcoinHsmProvider;
pub use security::SecurityManager;
pub use types::{HsmAuditEvent, HsmOperation};

#[cfg(test)]
pub mod tests;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use bitcoin::bip32::Xpriv;
use bitcoin::key::Secp256k1;
use bitcoin::opcodes::all as opcodes;
use bitcoin::taproot::TaprootBuilder;
use bitcoin::{Network, Psbt, Script, Txid, XOnlyPublicKey};
use chrono::{DateTime, Utc};
use secp256k1::ecdsa::Signature;
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info};

use self::config::HsmConfig;
use self::operations::OperationResponse;
use self::provider::{HsmProvider, HsmProviderType};

#[derive(Debug, Default, Clone)]
pub struct HsmStats {
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub last_operation_time: Option<DateTime<Utc>>,
    pub total_operation_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmStatus {
    Initializing,
    Ready,
    Error(String),
    Disconnected,
    ShuttingDown,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthStatus {
    pub last_check_time: Option<DateTime<Utc>>,
    pub last_check_result: bool,
    pub user_enabled: bool,
    pub last_upgrade_time: Option<DateTime<Utc>>,
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

pub struct HsmManager {
    config: HsmConfig,
    provider: Box<dyn HsmProvider>,
    stats: HsmStats,
    enabled: bool,
    audit_logger: Arc<AuditLogger>,
    status: Arc<RwLock<HsmStatus>>,
    health_status: Arc<RwLock<HsmHealthStatus>>,
    operation_tracker: Arc<Mutex<HashMap<String, (DateTime<Utc>, String)>>>,
}

impl HsmManager {
    pub async fn new(config: HsmConfig) -> Result<Self, HsmError> {
        info!(
            "Initializing HSM Manager with provider: {:?}",
            config.provider_type
        );

        let stats = HsmStats::default();
        let audit_logger = Arc::new(AuditLogger::new(&config.audit).await?);

        let provider: Box<dyn HsmProvider> = match config.provider_type {
            HsmProviderType::Simulator => Box::new(SimulatorHsmProvider::new(&config.simulator)?),
            HsmProviderType::SoftwareKeyStore => Box::new(
                SoftwareHsmProvider::new(
                    config.software.clone(),
                    Network::from(config.bitcoin.network),
                    audit_logger.clone(),
                )
                .await?,
            ),
            HsmProviderType::CloudHsm => {
                // CloudHsmProvider is not implemented yet
                return Err(HsmError::ProviderNotFound(
                    "CloudHsm provider not implemented".to_string(),
                ));
            }
            HsmProviderType::Hardware => Box::new(
                HardwareHsmProvider::new(
                    &config.hardware,
                    Network::from(config.bitcoin.network),
                    Arc::clone(&audit_logger),
                )
                .await?,
            ),
            HsmProviderType::Bitcoin => Box::new(BitcoinHsmProvider::new(&config.bitcoin).await?),
            HsmProviderType::Tpm => {
                return Err(HsmError::ProviderNotSupported(
                    "TPM provider not yet implemented".to_string(),
                ));
            }
            HsmProviderType::Pkcs11 => {
                return Err(HsmError::ProviderNotSupported(
                    "PKCS#11 provider not yet implemented".to_string(),
                ));
            }
            HsmProviderType::Custom => {
                return Err(HsmError::ProviderNotSupported(
                    "Custom provider not yet implemented".to_string(),
                ));
            }
        };

        Ok(Self {
            config,
            provider,
            stats,
            enabled: false,
            audit_logger,
            operation_tracker: Arc::new(Mutex::new(HashMap::new())),
            status: Arc::new(RwLock::new(HsmStatus::Initializing)),
            health_status: Arc::new(RwLock::new(HsmHealthStatus::default())),
        })
    }

    pub async fn initialize(&mut self) -> Result<(), HsmError> {
        info!("Initializing HSM Manager");
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Initializing;
        }
        self.audit_logger.initialize().await?;
        self.audit_logger
            .log_event(
                AuditEventType::HsmInitialize,
                AuditEventResult::InProgress,
                AuditEventSeverity::Info,
                serde_json::json!({
                    "event": "hsm.initialize",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "started",
                }),
            )
            .await?;
        self.provider
            .initialize()
            .await
            .map_err(|e| crate::security::hsm::error::HsmError::from(e))?;
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Ready;
        }
        self.audit_logger
            .log_event(
                AuditEventType::HsmInitialize,
                AuditEventResult::Success,
                AuditEventSeverity::Info,
                serde_json::json!({
                    "event": "hsm.initialize",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "completed",
                }),
            )
            .await?;
        info!(
            "HSM Manager initialized successfully with provider: {:?}",
            self.config.provider_type
        );
        Ok(())
    }

    pub async fn should_run_health_check(&self) -> bool {
        let health_status = self.health_status.read().await;
        match (
            health_status.last_upgrade_time,
            health_status.last_check_time,
        ) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(upgrade_time), Some(check_time)) => upgrade_time > check_time,
        }
    }

    pub async fn run_health_check(&mut self) -> Result<bool, HsmError> {
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Initializing;
        }
        self.audit_logger
            .log_event_legacy(
                "hsm.health_check",
                &crate::security::hsm::types::HsmAuditEvent {
                    event_type: "health_check".to_string(),
                    provider: format!("{:?}", self.config.provider_type),
                    status: "started".to_string(),
                    details: None,
                    operation_id: None,
                },
            )
            .await?;
        let check_result = self.provider.perform_health_check().await;
        {
            let mut health_status = self.health_status.write().await;
            health_status.last_check_time = Some(Utc::now());
            match &check_result {
                Ok(passed) => {
                    health_status.last_check_result = *passed;
                    if *passed {
                        if health_status.user_enabled {
                            health_status.disabled_reason = None;
                        } else {
                            health_status.disabled_reason =
                                Some("Waiting for user to enable HSM".to_string());
                        }
                    } else {
                        health_status.disabled_reason = Some("Health check failed".to_string());
                    }
                }
                Err(e) => {
                    health_status.last_check_result = false;
                    health_status.disabled_reason = Some(format!("Health check error: {}", e));
                }
            }
        }
        {
            let mut status = self.status.write().await;
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
        let result_str = match &check_result {
            Ok(true) => "passed",
            Ok(false) => "failed",
            Err(_) => "error",
        };
        let success = matches!(check_result, Ok(true));
        self.audit_logger
            .log_event(
                AuditEventType::HealthCheck,
                if success {
                    AuditEventResult::Success
                } else {
                    AuditEventResult::Failure
                },
                if success {
                    AuditEventSeverity::Info
                } else {
                    AuditEventSeverity::Error
                },
                serde_json::json!({
                    "event": "hsm.health_check",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "completed",
                    "details": format!("Result: {}", result_str),
                }),
            )
            .await?;
        check_result
    }

    pub async fn enable(&mut self) -> Result<(), HsmError> {
        if self.should_run_health_check().await {
            let health_check_result = self.run_health_check().await?;
            if !health_check_result {
                return Err(HsmError::NotReady(
                    "HSM failed health check and cannot be enabled".to_string(),
                ));
            }
        }
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Initializing;
        }
        {
            let mut health_status = self.health_status.write().await;
            health_status.user_enabled = true;
            health_status.disabled_reason = None;
        }
        self.audit_logger
            .log_event(
                AuditEventType::OperationRequest,
                AuditEventResult::InProgress,
                AuditEventSeverity::Info,
                serde_json::json!({
                    "event": "hsm.enable",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "started",
                    "details": "User-initiated enablement",
                }),
            )
            .await?;
        self.enabled = true;
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Ready;
        }
        self.audit_logger
            .log_event(
                AuditEventType::OperationRequest,
                AuditEventResult::Success,
                AuditEventSeverity::Info,
                serde_json::json!({
                    "event": "hsm.enable",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "completed",
                }),
            )
            .await?;
        Ok(())
    }

    pub async fn execute<T: Serialize + for<'de> Deserialize<'de> + Send + Sync>(
        &self,
        operation: HsmOperation,
        params: T,
    ) -> Result<operations::OperationResponse, HsmError> {
        let operation_id = format!("{}", uuid::Uuid::new_v4());
        debug!(
            "Executing HSM operation: {:?}, operation_id: {}",
            operation, operation_id
        );
        let request = HsmRequest {
            id: operation_id.clone(),
            operation: operation.clone(),
            parameters: serde_json::to_value(&params)?,
            user_id: None,
            timestamp: chrono::Utc::now(),
        };
        self.audit_logger
            .log_event(
                AuditEventType::HsmOperation,
                AuditEventResult::InProgress,
                AuditEventSeverity::Info,
                serde_json::json!({
                    "event": "hsm.operation",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "started",
                    "details": format!("operation_id: {}", operation_id.clone()),
                    "operation_id": operation_id.clone(),
                }),
            )
            .await?;
        {
            let status = self.status.read().await;
            if *status != HsmStatus::Ready {
                let err =
                    HsmError::NotReady(format!("HSM is not ready, current status: {:?}", *status));
                self.audit_logger
                    .log_event(
                        AuditEventType::HsmOperation,
                        AuditEventResult::Failure,
                        AuditEventSeverity::Error,
                        serde_json::json!({
                            "event": "hsm.operation",
                            "provider": format!("{:?}", self.config.provider_type),
                            "status": "failed",
                            "error": format!("{:?}", err),
                            "operation_id": operation_id,
                        }),
                    )
                    .await?;
                return Err(err);
            }
        }
        // Convert from types::HsmRequest to provider::HsmRequest
        let provider_request = provider::HsmRequest {
            id: request.id.clone(),
            operation: match request.operation {
                types::HsmOperation::GenerateKey => provider::HsmOperation::GenerateKey,
                types::HsmOperation::Sign => provider::HsmOperation::Sign,
                types::HsmOperation::Verify => provider::HsmOperation::Verify,
                types::HsmOperation::Encrypt => provider::HsmOperation::Encrypt,
                types::HsmOperation::Decrypt => provider::HsmOperation::Decrypt,
                types::HsmOperation::ExportPublicKey => provider::HsmOperation::ExportPublicKey,
                types::HsmOperation::ListKeys => provider::HsmOperation::ListKeys,
                types::HsmOperation::DeleteKey => provider::HsmOperation::DeleteKey,
                types::HsmOperation::GetStatus => provider::HsmOperation::GetStatus,
                types::HsmOperation::Custom(s) => provider::HsmOperation::Custom(s),
                _ => {
                    return Err(HsmError::InvalidOperation(
                        "Unsupported operation type".to_string(),
                    ))
                }
            },
            parameters: request.parameters,
        };

        match self.provider.execute_operation(provider_request).await {
            Ok(result) => {
                let _event = error::HsmAuditEvent::success(AuditEventType::HsmOperation)
                    .with_metadata(&serde_json::json!({
                        "provider": format!("{:?}", self.config.provider_type),
                        "action": "EXECUTE_OPERATION_SUCCESS",
                        "operation_id": operation_id
                    }))?;
                self.audit_logger
                    .log_event(
                        AuditEventType::HsmOperation,
                        AuditEventResult::Success,
                        AuditEventSeverity::Info,
                        serde_json::json!({
                            "event": "hsm.operation",
                            "provider": format!("{:?}", self.config.provider_type),
                            "status": "success",
                            "operation_id": operation_id,
                            "operation_data": serde_json::to_value(&operation).unwrap_or_default(),
                        }),
                    )
                    .await?;
                // Convert HsmResponse to OperationResponse
                let operation_response = OperationResponse {
                    status: operations::OperationStatus::Success,
                    data: result
                        .data
                        .map(|v| serde_json::to_vec(&v).unwrap_or_default()),
                    error: None,
                };
                Ok(operation_response)
            }
            Err(err) => {
                self.audit_logger
                    .log_event(
                        AuditEventType::HsmOperation,
                        AuditEventResult::Failure,
                        AuditEventSeverity::Error,
                        serde_json::json!({
                            "event": "hsm.operation",
                            "provider": format!("{:?}", self.config.provider_type),
                            "status": "failed",
                            "error": format!("{:?}", err),
                            "operation_id": operation_id,
                            "operation_data": serde_json::to_value(&operation).unwrap_or_default(),
                        }),
                    )
                    .await?;
                Err(err)
            }
        }
    }

    pub async fn generate_key_pair(
        &self,
        key_type: KeyType,
        key_name: &str,
    ) -> Result<KeyInfo, HsmError> {
        debug!("Generating key pair: {}, type: {:?}", key_name, key_type);
        let params = GenerateKeyParams {
            key_type,
            key_name: key_name.to_string(),
            store_in_hsm: true,
        };
        let result = self.execute(HsmOperation::GenerateKeyPair, params).await?;
        let data = result
            .data
            .ok_or_else(|| HsmError::InvalidData("No data in response".to_string()))?;
        let key_info: KeyInfo = serde_json::from_slice(&data)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(key_info)
    }

    pub async fn sign_data(
        &self,
        key_name: &str,
        data: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        debug!(
            "Signing data with key: {}, algorithm: {:?}",
            key_name, algorithm
        );
        let params = SignParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            algorithm,
        };
        let result = self.execute(HsmOperation::SignData, params).await?;
        let data = result.data.ok_or_else(|| {
            HsmError::DeserializationError("No signature data in response".to_string())
        })?;
        let signature_str = String::from_utf8(data).map_err(|e| {
            HsmError::DeserializationError(format!("Invalid UTF-8 in signature: {}", e))
        })?;
        let signature = BASE64
            .decode(signature_str.trim())
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(signature)
    }

    pub async fn verify_signature(
        &self,
        key_name: &str,
        data: &[u8],
        signature: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<bool, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        debug!(
            "Verifying signature with key: {}, algorithm: {:?}",
            key_name, algorithm
        );
        let params = VerifyParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            signature: BASE64.encode(signature),
            algorithm,
        };
        let result = self.execute(HsmOperation::VerifySignature, params).await?;
        let data = result.data.ok_or_else(|| {
            HsmError::DeserializationError("No verification result in response".to_string())
        })?;
        // Assume the result is a single byte representing boolean (0 = false, 1 = true)
        let verified = match data.first() {
            Some(0) => false,
            Some(1) => true,
            _ => {
                return Err(HsmError::DeserializationError(
                    "Invalid verification result format".to_string(),
                ))
            }
        };
        Ok(verified)
    }

    pub async fn encrypt_data(
        &self,
        key_name: &str,
        data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        debug!(
            "Encrypting data with key: {}, algorithm: {:?}",
            key_name, algorithm
        );
        let params = EncryptParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            algorithm,
        };
        let result = self.execute(HsmOperation::EncryptData, params).await?;
        let data_bytes = result.data.ok_or_else(|| {
            HsmError::DeserializationError("No encrypted data in response".to_string())
        })?;
        let data_str = String::from_utf8(data_bytes).map_err(|e| {
            HsmError::DeserializationError(format!("Invalid UTF-8 in encrypted data: {}", e))
        })?;
        let encrypted = BASE64
            .decode(data_str.trim())
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(encrypted)
    }

    pub async fn decrypt_data(
        &self,
        key_name: &str,
        data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        debug!(
            "Decrypting data with key: {}, algorithm: {:?}",
            key_name, algorithm
        );
        let params = DecryptParams {
            key_name: key_name.to_string(),
            data: BASE64.encode(data),
            algorithm,
        };
        let result = self.execute(HsmOperation::DecryptData, params).await?;
        let data_bytes = result.data.ok_or_else(|| {
            HsmError::DeserializationError("No decrypted data in response".to_string())
        })?;
        let data_str = String::from_utf8(data_bytes).map_err(|e| {
            HsmError::DeserializationError(format!("Invalid UTF-8 in decrypted data: {}", e))
        })?;
        let decrypted = BASE64
            .decode(data_str.trim())
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(decrypted)
    }

    pub async fn get_status(&self) -> HsmStatus {
        let status = self.status.read().await;
        (*status).clone()
    }

    pub async fn get_key_info(&self, key_name: &str) -> Result<KeyInfo, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        debug!("Getting key info for: {}", key_name);
        let params = GetKeyParams {
            key_name: key_name.to_string(),
        };
        let result = self.execute(HsmOperation::GetKeyInfo, params).await?;
        let data_bytes = result
            .data
            .ok_or_else(|| HsmError::DeserializationError("No key info in response".to_string()))?;
        let key_info: KeyInfo = serde_json::from_slice(&data_bytes)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(key_info)
    }

    pub async fn list_keys(&self) -> Result<Vec<KeyInfo>, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        debug!("Listing all keys");
        let result = self.execute(HsmOperation::ListKeys, ()).await?;
        let data_bytes = result.data.ok_or_else(|| {
            HsmError::DeserializationError("No keys data in response".to_string())
        })?;
        let keys: Vec<KeyInfo> = serde_json::from_slice(&data_bytes)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(keys)
    }

    pub async fn delete_key(&self, key_name: &str) -> Result<(), HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        info!("Deleting key: {}", key_name);
        let params = DeleteKeyParams {
            key_name: key_name.to_string(),
        };
        let _ = self.execute(HsmOperation::DeleteKey, params).await?;
        Ok(())
    }

    pub async fn get_audit_log(
        &self,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<crate::security::hsm::error::HsmAuditEvent>, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        debug!("Getting audit log");
        let events = self
            .audit_logger
            .get_events(start_time, end_time, limit)
            .await?
            .into_iter()
            .map(|event| event.to_hsm_audit_event())
            .collect();
        Ok(events)
    }

    pub async fn rotate_key(&self, key_name: &str) -> Result<KeyInfo, HsmError> {
        if !self.enabled {
            return Err(HsmError::Disabled("HSM is not enabled".to_string()));
        }
        info!("Rotating key: {}", key_name);
        let params = RotateKeyParams {
            key_name: key_name.to_string(),
        };
        let result = self.execute(HsmOperation::RotateKey, params).await?;
        let data_bytes = result
            .data
            .ok_or_else(|| HsmError::DeserializationError("No key info in response".to_string()))?;
        let key_info: KeyInfo = serde_json::from_slice(&data_bytes)
            .map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        Ok(key_info)
    }

    pub async fn disable(&mut self) -> Result<(), HsmError> {
        self.enabled = false;
        *self.status.write().await = HsmStatus::Disabled;
        self.audit_logger
            .log_event(
                AuditEventType::HsmOperation,
                AuditEventResult::Success,
                AuditEventSeverity::Info,
                serde_json::json!({
                    "event": "hsm.disable",
                    "provider": format!("{:?}", self.config.provider_type),
                    "status": "completed",
                    "details": "HSM disabled by user",
                }),
            )
            .await?;
        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtomicSwap {
    preimage_hash: Sha256Hash,
    timeout: u32,
    amount: u64,
    redeem_script: ScriptBuf,
}

pub struct NetworkManager {
    connected: bool,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self { connected: false }
    }

    pub async fn get_merkle_proof(&self, _txid: &Txid) -> Result<MerkleProof, Box<dyn Error>> {
        Ok(MerkleProof {
            path: vec![[0u8; 32], [1u8; 32]],
            indices: vec![false, true],
        })
    }

    pub async fn initiate_swap(
        &self,
        amount: u64,
        _counterparty: &str,
    ) -> Result<AtomicSwap, Box<dyn std::error::Error>> {
        use bitcoin::blockdata::script::Builder;
        use bitcoin::opcodes;
        use sha2::{Digest, Sha256};
        use std::convert::TryInto;

        let preimage = [0u8; 32];
        let mut hasher = Sha256::new();
        hasher.update(&preimage);
        let hash = hasher.finalize();
        let hash_array: [u8; 32] = hash
            .as_slice()
            .try_into()
            .map_err(|_| "Invalid hash length")?;
        let hash_wrapper = Sha256Hash { hash: hash_array };

        let mut builder = Builder::new()
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(&hash_wrapper.hash)
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_ELSE)
            .push_int(0)
            .push_opcode(opcodes::all::OP_VERIFY);

        let counterparty_key = [0u8; 33];
        builder = builder
            .push_slice(&counterparty_key)
            .push_opcode(opcodes::all::OP_ENDIF)
            .push_opcode(opcodes::all::OP_CHECKSIG);

        let script = builder.into_script();

        Ok(AtomicSwap {
            preimage_hash: hash_wrapper,
            timeout: 144,
            amount,
            redeem_script: script,
        })
    }
}

impl SecurityManager {
    pub fn create_multisig_wallet(
        &self,
        threshold: usize,
        keys: &[XOnlyPublicKey],
    ) -> Result<String, HsmError> {
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
        let spend_info = builder
            .finalize(&secp, internal_key)
            .map_err(|_| HsmError::InvalidKey("Failed to finalize taproot builder".to_string()))?;
        Ok(spend_info.output_key().to_string())
    }

    pub fn gpu_resistant_derive(&self, mnemonic: &str) -> Result<Xpriv, Box<dyn Error>> {
        use argon2::{Algorithm, Argon2, Params, Version};
        use bitcoin::Network;

        let salt = "ANYA_CORE_SALT_V2";
        let params =
            Params::new(15000, 2, 1, Some(32)).map_err(|e| -> Box<dyn std::error::Error> {
                Box::new(Argon2Error::Error(e.to_string()))
            })?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut output_key = [0u8; 32];
        argon2
            .hash_password_into(mnemonic.as_bytes(), salt.as_bytes(), &mut output_key)
            .map_err(|e| -> Box<dyn std::error::Error> {
                Box::new(Argon2Error::Error(e.to_string()))
            })?;
        Xpriv::new_master(Network::Bitcoin, &output_key)
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepudiationProof {
    nonce: [u8; 32],
    partial_sig: Signature,
    merkle_proof: MerkleProof,
}

pub struct MobileSDK {
    security: Arc<SecurityManager>,
    network: Arc<NetworkManager>,
}

impl MobileSDK {
    pub async fn generate_repudiation_proof(
        &self,
        txid: &Txid,
    ) -> Result<RepudiationProof, Box<dyn std::error::Error>> {
        use rand::RngCore;

        let mut nonce = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut nonce);

        // The SecurityManager doesn't have sign_data, we should use the HSM manager directly
        // or implement the sign_data method on SecurityManager
        let partial_sig_bytes = self
            .security
            .sign_repudiation(txid, &nonce)
            .await?
            .serialize_compact();

        let merkle_proof = self.network.get_merkle_proof(txid).await?;

        let partial_sig = Signature::from_compact(&partial_sig_bytes)?;

        Ok(RepudiationProof {
            nonce,
            partial_sig,
            merkle_proof,
        })
    }
}

pub struct HsmBridge {
    provider: Box<dyn HsmProvider>,
    connected: bool,
}

impl HsmBridge {
    pub async fn sign_transaction(&mut self, psbt: &mut Psbt) -> Result<(), HsmError> {
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

impl<'de, T: serde::Serialize + serde::de::DeserializeOwned> serde::Deserialize<'de>
    for CoreWrapper<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|inner| CoreWrapper { inner })
    }
}

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
