pub mod provider;
pub mod audit;
pub mod config;
pub mod operations;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, debug, warn};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Mutex;

use self::config::HsmConfig;
use self::provider::{HsmProvider, HsmProviderType};
use self::audit::AuditLogger;
use self::operations::{HsmOperation, OperationResult, OperationRequest};

/// HSM Manager that provides a unified interface to hardware security modules
/// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
pub struct HsmManager {
    /// Configuration for the HSM
    pub config: HsmConfig,
    
    /// Active HSM provider
    pub provider: Box<dyn HsmProvider>,
    
    /// Audit logger for HSM operations
    pub audit_logger: Arc<AuditLogger>,
    
    /// Current status
    pub status: Arc<RwLock<HsmStatus>>,
    
    /// Operation tracker
    pub operation_tracker: Arc<Mutex<HashMap<String, (DateTime<Utc>, String)>>>,
}

impl HsmManager {
    /// Creates a new HSM Manager with the specified configuration
    pub async fn new(config: HsmConfig) -> Result<Self, HsmError> {
        info!("Initializing HSM Manager with provider: {:?}", config.provider_type);
        
        // Create the provider based on configuration
        let provider: Box<dyn HsmProvider> = match config.provider_type {
            HsmProviderType::Simulator => Box::new(SimulatorHsmProvider::new(&config.simulator)?),
            HsmProviderType::SoftwareKeyStore => Box::new(SoftwareHsmProvider::new(&config.software)?),
            HsmProviderType::CloudHsm => Box::new(CloudHsmProvider::new(&config.cloud).await?),
            HsmProviderType::HardwareHsm => Box::new(HardwareHsmProvider::new(&config.hardware).await?),
            HsmProviderType::BitcoinHsm => Box::new(BitcoinHsmProvider::new(&config.bitcoin).await?),
        };
        
        // Create audit logger
        let audit_logger = Arc::new(AuditLogger::new(&config.audit).await?);
        
        // Create the HSM manager
        let manager = Self {
            config,
            provider,
            audit_logger,
            operation_tracker: Arc::new(Mutex::new(HashMap::new())),
            status: Arc::new(RwLock::new(HsmStatus::Initializing)),
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
        self.audit_logger.log_event(
            "hsm.initialize",
            &HsmAuditEvent {
                event_type: "initialize".to_string(),
                provider: format!("{:?}", self.config.provider_type),
                status: "started".to_string(),
                details: None,
                operation_id: None,
            }
        ).await?;
        
        // Initialize the provider
        self.provider.initialize().await?;
        
        // Update status
        {
            let mut status = self.status.write().await;
            *status = HsmStatus::Ready;
        }
        
        // Log successful initialization
        self.audit_logger.log_event(
            "hsm.initialize",
            &HsmAuditEvent {
                event_type: "initialize".to_string(),
                provider: format!("{:?}", self.config.provider_type),
                status: "success".to_string(),
                details: None,
                operation_id: None,
            }
        ).await?;
        
        info!("HSM Manager initialized successfully with provider: {:?}", self.config.provider_type);
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
        debug!("Executing HSM operation: {:?}, operation_id: {}", operation, operation_id);
        
        // Log operation start
        self.audit_logger.log_event(
            "hsm.operation",
            &HsmAuditEvent {
                event_type: "operation_start".to_string(),
                provider: format!("{:?}", self.config.provider_type),
                status: "started".to_string(),
                details: Some(format!("Operation: {:?}", operation)),
                operation_id: Some(operation_id.clone()),
            }
        ).await?;
        
        // Check HSM status
        {
            let status = self.status.read().await;
            if *status != HsmStatus::Ready {
                let err = HsmError::NotReady(format!("HSM is not ready, current status: {:?}", *status));
                
                // Log operation failure
                self.audit_logger.log_event(
                    "hsm.operation",
                    &HsmAuditEvent {
                        event_type: "operation_error".to_string(),
                        provider: format!("{:?}", self.config.provider_type),
                        status: "failed".to_string(),
                        details: Some(format!("Error: {:?}", err)),
                        operation_id: Some(operation_id),
                    }
                ).await?;
                
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
                self.audit_logger.log_event(
                    "hsm.operation",
                    &HsmAuditEvent {
                        event_type: "operation_complete".to_string(),
                        provider: format!("{:?}", self.config.provider_type),
                        status: "success".to_string(),
                        details: None,
                        operation_id: Some(operation_id),
                    }
                ).await?;
                
                Ok(result)
            },
            Err(err) => {
                // Log operation failure
                self.audit_logger.log_event(
                    "hsm.operation",
                    &HsmAuditEvent {
                        event_type: "operation_error".to_string(),
                        provider: format!("{:?}", self.config.provider_type),
                        status: "failed".to_string(),
                        details: Some(format!("Error: {:?}", err)),
                        operation_id: Some(operation_id),
                    }
                ).await?;
                
                Err(err)
            }
        };
        
        result
    }
    
    /// Generates a new key pair
    pub async fn generate_key_pair(&self, key_type: KeyType, key_name: &str) -> Result<KeyInfo, HsmError> {
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
    pub async fn sign_data(&self, key_name: &str, data: &[u8], algorithm: SignatureAlgorithm) -> Result<Vec<u8>, HsmError> {
        debug!("Signing data with key: {}, algorithm: {:?}", key_name, algorithm);
        
        // Call the execute method with SignData operation
        let params = SignParams {
            key_name: key_name.to_string(),
            data: base64::encode(data),
            algorithm,
        };
        
        let result = self.execute(HsmOperation::SignData, params).await?;
        
        // Convert result to signature bytes
        let signature = base64::decode(
            result.data.as_str()
                .ok_or_else(|| HsmError::DeserializationError("Expected string for signature".to_string()))?
        ).map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        
        Ok(signature)
    }
    
    /// Verifies a signature using a key stored in the HSM
    pub async fn verify_signature(
        &self,
        key_name: &str,
        data: &[u8],
        signature: &[u8],
        algorithm: SignatureAlgorithm,
    ) -> Result<bool, HsmError> {
        debug!("Verifying signature with key: {}, algorithm: {:?}", key_name, algorithm);
        
        // Call the execute method with VerifySignature operation
        let params = VerifyParams {
            key_name: key_name.to_string(),
            data: base64::encode(data),
            signature: base64::encode(signature),
            algorithm,
        };
        
        let result = self.execute(HsmOperation::VerifySignature, params).await?;
        
        // Convert result to boolean
        let verified = result.data.as_bool()
            .ok_or_else(|| HsmError::DeserializationError("Expected boolean for verification result".to_string()))?;
            
        Ok(verified)
    }
    
    /// Encrypts data using a key stored in the HSM
    pub async fn encrypt_data(
        &self,
        key_name: &str,
        data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        debug!("Encrypting data with key: {}, algorithm: {:?}", key_name, algorithm);
        
        // Call the execute method with EncryptData operation
        let params = EncryptParams {
            key_name: key_name.to_string(),
            data: base64::encode(data),
            algorithm,
        };
        
        let result = self.execute(HsmOperation::EncryptData, params).await?;
        
        // Convert result to encrypted bytes
        let encrypted = base64::decode(
            result.data.as_str()
                .ok_or_else(|| HsmError::DeserializationError("Expected string for encrypted data".to_string()))?
        ).map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        
        Ok(encrypted)
    }
    
    /// Decrypts data using a key stored in the HSM
    pub async fn decrypt_data(
        &self,
        key_name: &str,
        data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, HsmError> {
        debug!("Decrypting data with key: {}, algorithm: {:?}", key_name, algorithm);
        
        // Call the execute method with DecryptData operation
        let params = DecryptParams {
            key_name: key_name.to_string(),
            data: base64::encode(data),
            algorithm,
        };
        
        let result = self.execute(HsmOperation::DecryptData, params).await?;
        
        // Convert result to decrypted bytes
        let decrypted = base64::decode(
            result.data.as_str()
                .ok_or_else(|| HsmError::DeserializationError("Expected string for decrypted data".to_string()))?
        ).map_err(|e| HsmError::DeserializationError(e.to_string()))?;
        
        Ok(decrypted)
    }
    
    /// Gets the current HSM status
    pub async fn get_status(&self) -> HsmStatus {
        let status = self.status.read().await;
        status.clone()
    }
    
    /// Gets information about a key stored in the HSM
    pub async fn get_key_info(&self, key_name: &str) -> Result<KeyInfo, HsmError> {
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
        debug!("Getting audit log");
        
        // Delegate to the audit logger
        let events = self.audit_logger.get_events(start_time, end_time, limit).await?;
        
        Ok(events)
    }
    
    /// Rotates a key in the HSM
    pub async fn rotate_key(&self, key_name: &str) -> Result<KeyInfo, HsmError> {
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
}

/// Type of key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    /// RSA key pair
    Rsa(RsaKeySize),
    
    /// Elliptic curve key pair
    Ec(EcCurve),
    
    /// AES symmetric key
    Aes(AesKeySize),
    
    /// Ed25519 key pair
    Ed25519,
}

/// RSA key size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RsaKeySize {
    /// 2048 bits
    Bits2048,
    
    /// 3072 bits
    Bits3072,
    
    /// 4096 bits
    Bits4096,
}

/// Elliptic curve type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EcCurve {
    /// P-256 curve (secp256r1)
    P256,
    
    /// P-384 curve (secp384r1)
    P384,
    
    /// P-521 curve (secp521r1)
    P521,
    
    /// Secp256k1 curve (used in Bitcoin)
    Secp256k1,
}

/// AES key size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AesKeySize {
    /// 128 bits
    Bits128,
    
    /// 192 bits
    Bits192,
    
    /// 256 bits
    Bits256,
}

/// Information about a key stored in the HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Name of the key
    pub name: String,
    
    /// Type of the key
    pub key_type: KeyType,
    
    /// Identifier for the key in the HSM
    pub hsm_id: String,
    
    /// Creation time
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Last used time
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Public key (for asymmetric keys)
    pub public_key: Option<String>,
    
    /// Key version
    pub version: u32,
    
    /// Whether the key can be exported
    pub exportable: bool,
}

/// Parameters for generating a key
#[derive(Debug, Serialize, Deserialize)]
struct GenerateKeyParams {
    /// Type of key to generate
    pub key_type: KeyType,
    
    /// Name to give the key
    pub key_name: String,
    
    /// Whether to store the key in the HSM
    pub store_in_hsm: bool,
}

/// Parameters for signing data
#[derive(Debug, Serialize, Deserialize)]
struct SignParams {
    /// Name of the key to use
    pub key_name: String,
    
    /// Data to sign (base64 encoded)
    pub data: String,
    
    /// Signature algorithm to use
    pub algorithm: SignatureAlgorithm,
}

/// Parameters for verifying a signature
#[derive(Debug, Serialize, Deserialize)]
struct VerifyParams {
    /// Name of the key to use
    pub key_name: String,
    
    /// Data that was signed (base64 encoded)
    pub data: String,
    
    /// Signature to verify (base64 encoded)
    pub signature: String,
    
    /// Signature algorithm used
    pub algorithm: SignatureAlgorithm,
}

/// Parameters for encrypting data
#[derive(Debug, Serialize, Deserialize)]
struct EncryptParams {
    /// Name of the key to use
    pub key_name: String,
    
    /// Data to encrypt (base64 encoded)
    pub data: String,
    
    /// Encryption algorithm to use
    pub algorithm: EncryptionAlgorithm,
}

/// Parameters for decrypting data
#[derive(Debug, Serialize, Deserialize)]
struct DecryptParams {
    /// Name of the key to use
    pub key_name: String,
    
    /// Data to decrypt (base64 encoded)
    pub data: String,
    
    /// Encryption algorithm used
    pub algorithm: EncryptionAlgorithm,
}

/// Parameters for getting key info
#[derive(Debug, Serialize, Deserialize)]
struct GetKeyParams {
    /// Name of the key
    pub key_name: String,
}

/// Parameters for deleting a key
#[derive(Debug, Serialize, Deserialize)]
struct DeleteKeyParams {
    /// Name of the key to delete
    pub key_name: String,
}

/// Parameters for rotating a key
#[derive(Debug, Serialize, Deserialize)]
struct RotateKeyParams {
    /// Name of the key to rotate
    pub key_name: String,
}

/// Signature algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    /// RSA with PKCS#1 v1.5 padding and SHA-256
    RsaPkcs1v15Sha256,
    
    /// RSA with PKCS#1 v1.5 padding and SHA-384
    RsaPkcs1v15Sha384,
    
    /// RSA with PKCS#1 v1.5 padding and SHA-512
    RsaPkcs1v15Sha512,
    
    /// RSA with PSS padding and SHA-256
    RsaPssSha256,
    
    /// RSA with PSS padding and SHA-384
    RsaPssSha384,
    
    /// RSA with PSS padding and SHA-512
    RsaPssSha512,
    
    /// ECDSA with SHA-256
    EcdsaSha256,
    
    /// ECDSA with SHA-384
    EcdsaSha384,
    
    /// ECDSA with SHA-512
    EcdsaSha512,
    
    /// Ed25519
    Ed25519,
}

/// Encryption algorithm
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// RSA PKCS#1 v1.5
    RsaPkcs1,
    /// RSA OAEP with SHA-256
    RsaOaepSha256,
    /// AES-CBC with 256-bit key
    AesCbc256,
    /// AES-GCM with 256-bit key
    AesGcm256,
    /// AES-CTR with 256-bit key
    AesCtr256,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
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
}

/// HSM audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAuditEvent {
    /// Type of event
    pub event_type: String,
    
    /// HSM provider
    pub provider: String,
    
    /// Status of the event
    pub status: String,
    
    /// Additional details
    pub details: Option<String>,
    
    /// Operation ID (if applicable)
    pub operation_id: Option<String>,
}

/// HSM errors
#[derive(Debug, thiserror::Error)]
pub enum HsmError {
    #[error("HSM provider error: {0}")]
    ProviderError(String),
    
    #[error("HSM not ready: {0}")]
    NotReady(String),
    
    #[error("HSM operation error: {0}")]
    OperationError(String),
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Audit logging error: {0}")]
    AuditError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Drop for HsmManager {
    fn drop(&mut self) {
        // This is a best-effort attempt to clean up. Since we can't use async in Drop,
        // we just log a warning if we haven't closed properly.
        let status = futures::executor::block_on(self.status.read());
        if *status != HsmStatus::ShuttingDown && *status != HsmStatus::Initializing {
            warn!("HsmManager dropped without calling close(). Resources may not be cleaned up properly.");
        }
    }
}

// Ensure the HsmProvider trait has an execute_operation method
pub trait HsmProvider: Send + Sync {
    // ... existing methods ...
    
    async fn execute_operation(&self, request: HsmRequest) -> Result<HsmResponse, HsmError>;
    
    // ... existing methods ...
}

// HSM Integration Core
#[derive(Debug, Clone)]
pub struct HsmClient {
    connector: Arc<dyn HsmConnector>,
    network: Network,
}

#[async_trait]
pub trait HsmConnector: Send + Sync {
    async fn sign_taproot(&self, msg: &[u8], path: &HsmKeyPath) -> Result<Signature>;
    async fn derive_key(&self, path: &HsmKeyPath) -> Result<XOnlyPublicKey>;
    async fn get_pubkey(&self, path: &HsmKeyPath) -> Result<XOnlyPublicKey>;
}

impl SecurityManager {
    pub async fn new_hsm_client(&self, hsm_type: HsmType) -> Result<HsmClient> {
        match hsm_type {
            HsmType::YubiHsm => YubiConnector::new().await,
            HsmType::Ledger => LedgerConnector::new().await,
        }
    }
}

// Lightning Atomic Swaps
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomicSwap {
    preimage_hash: Sha256,
    timeout: u32,
    amount: u64,
    redeem_script: ScriptBuf,
}

impl NetworkManager {
    pub async fn initiate_swap(&self, amount: u64, counterparty: &str) -> Result<AtomicSwap> {
        let preimage = rand::thread_rng().gen::<[u8; 32]>();
        let hash = Sha256::hash(&preimage);
        
        let script = Builder::new()
            .push_opcode(opcodes::OP_IF)
            .push_slice(&hash)
            .push_opcode(opcodes::OP_ELSE)
            .push_int(self.network.get_block_height()? + 144)
            .push_opcode(opcodes::OP_CHECKSEQUENCEVERIFY)
            .push_opcode(opcodes::OP_DROP)
            .push_slice(&counterparty)
            .push_opcode(opcodes::OP_ENDIF)
            .push_opcode(opcodes::OP_CHECKSIG)
            .into_script();
        
        Ok(AtomicSwap {
            preimage_hash: hash,
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
        keys: &[XOnlyPublicKey]
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
    pub fn gpu_resistant_derive(&self, mnemonic: &str) -> Result<ExtendedPrivKey> {
        let salt = "ANYA_CORE_SALT_V2";
        let mut kdf = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(15000, 2, 1, Some(32))?,
        );
        
        let seed = kdf.hash_password(mnemonic.as_bytes(), salt.as_bytes())?;
        ExtendedPrivKey::new_master(Network::Bitcoin, &seed.hash)
    }
}

// Transaction Repudiation Proofs
#[derive(Debug, Serialize, Deserialize)]
pub struct RepudiationProof {
    nonce: [u8; 32],
    partial_sig: Signature,
    merkle_proof: MerkleProof,
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