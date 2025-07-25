use bitcoin::hashes::*;
// [AIR-3][AIS-3][BPC-3][RES-3] Import necessary dependencies for HSM types
// This follows official Bitcoin Improvement Proposals (BIPs) standards for secure HSM implementation
use chrono::{DateTime, Utc};
use secp256k1::ecdsa::Signature;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fmt;
use uuid::Uuid;

/// Key pair generated by the HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    /// Key ID
    pub id: String,
    /// Key type
    pub key_type: KeyType,
    /// Public key data
    pub public_key: Vec<u8>,
    /// Private key handle (not the actual private key)
    pub private_key_handle: String,
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

/// HSM operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmOperation {
    /// Generate a key pair
    GenerateKey,
    /// Sign data with a key
    Sign,
    /// Verify a signature
    Verify,
    /// Encrypt data
    Encrypt,
    /// Decrypt data
    Decrypt,
    /// Export a public key
    ExportPublicKey,
    /// List all keys
    ListKeys,
    /// Delete a key
    DeleteKey,
    /// Get HSM provider status
    GetStatus,
    /// Custom operation
    Custom(String),
    /// Get key information
    GetKeyInfo,
    /// Rotate a key
    RotateKey,

    /// Generate a key pair (alias for GenerateKey)
    GenerateKeyPair,

    /// Sign data
    SignData,

    /// Verify a signature
    VerifySignature,

    /// Encrypt data
    EncryptData,

    /// Decrypt data
    DecryptData,
}

/// HSM request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmRequest {
    /// Request ID
    pub id: String,
    /// Operation to perform
    pub operation: HsmOperation,
    /// Parameters for the operation (encoded as JSON)
    pub parameters: serde_json::Value,
    /// User ID making the request
    pub user_id: Option<String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

impl HsmRequest {
    /// Create a new HSM request
    pub fn new(operation: HsmOperation, parameters: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            operation,
            parameters,
            user_id: None,
            timestamp: Utc::now(),
        }
    }

    /// Set the user ID
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
}

/// HSM response status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmResponseStatus {
    /// Operation was successful
    Success,
    /// Operation failed
    Failure,
    /// Operation is still in progress
    InProgress,
}

/// HSM response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmResponse {
    /// Request ID that this response is for
    pub request_id: String,
    /// Response status
    pub status: HsmResponseStatus,
    /// Response data (encoded as JSON)
    pub data: Option<serde_json::Value>,
    /// Error message (if any)
    pub error: Option<String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

impl HsmResponse {
    /// Create a new successful HSM response
    pub fn success(request_id: impl Into<String>, data: Option<serde_json::Value>) -> Self {
        Self {
            request_id: request_id.into(),
            status: HsmResponseStatus::Success,
            data,
            error: None,
            timestamp: Utc::now(),
        }
    }

    /// Create a new failure HSM response
    pub fn failure(request_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            status: HsmResponseStatus::Failure,
            data: None,
            error: Some(error.into()),
            timestamp: Utc::now(),
        }
    }

    /// Create a new in-progress HSM response
    pub fn in_progress(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            status: HsmResponseStatus::InProgress,
            data: None,
            error: None,
            timestamp: Utc::now(),
        }
    }
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
    pub created_at: DateTime<Utc>,
    /// Last used time
    pub last_used: Option<DateTime<Utc>>,
    /// Public key (for asymmetric keys)
    pub public_key: Option<String>,
    /// Key version
    pub version: u32,
    /// Whether the key can be exported
    pub exportable: bool,
}

/// Parameters for generating a key
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateKeyParams {
    /// Type of key to generate
    pub key_type: KeyType,
    /// Name to give the key
    pub key_name: String,
    /// Whether to store the key in the HSM
    pub store_in_hsm: bool,
}

/// Parameters for signing data
#[derive(Debug, Serialize, Deserialize)]
pub struct SignParams {
    /// Name of the key to use
    pub key_name: String,
    /// Data to sign (base64 encoded)
    pub data: String,
    /// Signature algorithm to use
    pub algorithm: SignatureAlgorithm,
}

/// Parameters for verifying a signature
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyParams {
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
pub struct EncryptParams {
    /// Name of the key to use
    pub key_name: String,
    /// Data to encrypt (base64 encoded)
    pub data: String,
    /// Encryption algorithm to use
    pub algorithm: EncryptionAlgorithm,
}

/// Parameters for decrypting data
#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptParams {
    /// Name of the key to use
    pub key_name: String,
    /// Data to decrypt (base64 encoded)
    pub data: String,
    /// Encryption algorithm used
    pub algorithm: EncryptionAlgorithm,
}

/// Parameters for getting key info
#[derive(Debug, Serialize, Deserialize)]
pub struct GetKeyParams {
    /// Name of the key
    pub key_name: String,
}

/// Parameters for deleting a key
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteKeyParams {
    /// Name of the key to delete
    pub key_name: String,
}

/// Parameters for rotating a key
#[derive(Debug, Serialize, Deserialize)]
pub struct RotateKeyParams {
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

    /// ECDSA with SHA-256 (alias for secp256k1)
    EcdsaSha256,
    EcdsaSecp256k1Sha256,

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

/// HSM key path for hardware modules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HsmKeyPath {
    /// Path components
    pub components: Vec<u32>,
    /// Hardened flags for each component
    pub hardened: Vec<bool>,
}

impl HsmKeyPath {
    /// Create a new key path from components and hardened flags
    pub fn new(components: Vec<u32>, hardened: Vec<bool>) -> Self {
        assert_eq!(components.len(), hardened.len());
        Self {
            components,
            hardened,
        }
    }

    /// Parse a key path string like "m/44'/0'/0'/0/0"
    pub fn from_string(s: &str) -> Result<Self, Box<dyn Error>> {
        if !s.starts_with("m/") {
            return Err(format!("Invalid key path: {}", s).into());
        }

        let s = &s[2..]; // Skip the "m/"
        let mut components = Vec::new();
        let mut hardened = Vec::new();

        for part in s.split('/') {
            if part.is_empty() {
                continue;
            }

            let is_hardened = part.ends_with('\'') || part.ends_with('h');
            let num_str = if is_hardened {
                &part[..part.len() - 1]
            } else {
                part
            };

            let num = num_str.parse::<u32>()?;
            components.push(num);
            hardened.push(is_hardened);
        }

        Ok(Self::new(components, hardened))
    }

    /// Format the key path as a string
    pub fn to_string(&self) -> String {
        let mut result = String::from("m");

        for i in 0..self.components.len() {
            result.push('/');
            result.push_str(&self.components[i].to_string());
            if self.hardened[i] {
                result.push('\'');
            }
        }

        result
    }
}

impl fmt::Display for HsmKeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Merkle proof for blockchain validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Hash path
    pub path: Vec<[u8; 32]>,
    /// Index bits
    pub indices: Vec<bool>,
}

/// Repudiation proof for mobile SDKs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepudiationProof {
    /// Partial signature
    pub partial_sig: Signature,
    /// Merkle proof
    pub merkle_proof: MerkleProof,
}

/// User type for security operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmType {
    /// YubiHSM hardware device
    YubiHsm,
    /// Ledger hardware wallet
    Ledger,
    /// Software implementation (for testing)
    Software,
}

/// HSM key usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyUsage {
    /// Signing
    Signing,
    /// Encryption
    Encryption,
    /// Key wrapping
    KeyWrapping,
    /// Authentication
    Authentication,
}

/// HSM operations that can be performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenParams {
    /// Type of key to generate
    pub key_type: KeyType,
    /// Usage of the key
    pub key_usage: Vec<KeyUsage>,
    /// Label for the key
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct CoreWrapper<T> {
    // Add fields as needed
    pub data: Option<T>,
}

// Add conversion from provider::SigningAlgorithm to types::SignatureAlgorithm
impl From<crate::security::hsm::provider::SigningAlgorithm> for SignatureAlgorithm {
    fn from(algorithm: crate::security::hsm::provider::SigningAlgorithm) -> Self {
        use crate::security::hsm::provider::SigningAlgorithm as ProviderAlgorithm;
        match algorithm {
            ProviderAlgorithm::RsaPkcs1Sha256 => SignatureAlgorithm::RsaPkcs1v15Sha256,
            ProviderAlgorithm::RsaPkcs1Sha384 => SignatureAlgorithm::RsaPkcs1v15Sha384,
            ProviderAlgorithm::RsaPkcs1Sha512 => SignatureAlgorithm::RsaPkcs1v15Sha512,
            ProviderAlgorithm::RsaPssSha256 => SignatureAlgorithm::RsaPssSha256,
            ProviderAlgorithm::RsaPssSha384 => SignatureAlgorithm::RsaPssSha384,
            ProviderAlgorithm::RsaPssSha512 => SignatureAlgorithm::RsaPssSha512,
            ProviderAlgorithm::EcdsaSha256 => SignatureAlgorithm::EcdsaSecp256k1Sha256,
            ProviderAlgorithm::Ed25519 => SignatureAlgorithm::Ed25519,
            _ => SignatureAlgorithm::EcdsaSecp256k1Sha256, // Default for unsupported algorithms
        }
    }
}

// Add conversion from types::SignatureAlgorithm to provider::SigningAlgorithm
impl From<SignatureAlgorithm> for crate::security::hsm::provider::SigningAlgorithm {
    fn from(algorithm: SignatureAlgorithm) -> Self {
        use crate::security::hsm::provider::SigningAlgorithm as ProviderAlgorithm;
        match algorithm {
            SignatureAlgorithm::RsaPkcs1v15Sha256 => ProviderAlgorithm::RsaPkcs1Sha256,
            SignatureAlgorithm::RsaPkcs1v15Sha384 => ProviderAlgorithm::RsaPkcs1Sha384,
            SignatureAlgorithm::RsaPkcs1v15Sha512 => ProviderAlgorithm::RsaPkcs1Sha512,
            SignatureAlgorithm::RsaPssSha256 => ProviderAlgorithm::RsaPssSha256,
            SignatureAlgorithm::RsaPssSha384 => ProviderAlgorithm::RsaPssSha384,
            SignatureAlgorithm::RsaPssSha512 => ProviderAlgorithm::RsaPssSha512,
            SignatureAlgorithm::EcdsaSecp256k1Sha256 => ProviderAlgorithm::EcdsaSha256,
            SignatureAlgorithm::Ed25519 => ProviderAlgorithm::Ed25519,
            _ => ProviderAlgorithm::EcdsaSha256, // Default for unsupported algorithms
        }
    }
}

// Implement Serialize and Deserialize manually to avoid generic type parameter issues
impl<T> Serialize for CoreWrapper<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_struct("CoreWrapper", 1)?;
        if let Some(data) = &self.data {
            map.serialize_field("data", data)?;
        }
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for CoreWrapper<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CoreWrapperHelper<T> {
            data: Option<T>,
        }

        let helper = CoreWrapperHelper::deserialize(deserializer)?;
        Ok(CoreWrapper { data: helper.data })
    }
}
