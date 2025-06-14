use std::error::Error;
// Web5 Identity Implementation
// Provides DID (Decentralized Identity) functionality
// as part of the Web5 integration - [AIR-012] Operational Reliability

use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// Define Result type for Web5
pub type Web5Result<T> = Result<T, Web5Error>;

// Define Error enum for Web5
#[derive(Debug, thiserror::Error)]
pub enum Web5Error {
    #[error("Identity error: {0}")]
    Identity(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Communication error: {0}")]
    Communication(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Credential error: {0}")]
    Credential(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("DWN error: {0}")]
    DWNError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

// [AIS-3] Implementation for From<Box<dyn std::error::Error>> for Web5Error
// This allows the ? operator to work correctly when converting from Box<dyn Error> to Web5Error
impl From<Box<dyn std::error::Error>> for Web5Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Web5Error::Protocol(err.to_string())
    }
}

// [AIS-3] Implementation for From<String> for Web5Error
// This allows the ? operator to work correctly when converting from String to Web5Error
impl From<String> for Web5Error {
    fn from(err: String) -> Self {
        Web5Error::Protocol(err)
    }
}

// [AIS-3] Implementation for From<&str> for Web5Error
// This allows the ? operator to work correctly when converting from &str to Web5Error
impl From<&str> for Web5Error {
    fn from(err: &str) -> Self {
        Web5Error::Protocol(err.to_string())
    }
}

/// DID Manager
///
/// Core component responsible for decentralized identity management.
/// Implements the ports and adapters pattern for extensibility.
#[derive(Clone, Debug)]
pub struct DIDManager {
    /// DIDs managed by this instance
    dids: Arc<Mutex<HashMap<String, DID>>>,
    /// Default DID to use
    default_did: Option<String>,
    /// DID method to use
    method: String,
}

/// Decentralized Identifier
///
/// Represents a DID with its document and private keys.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DID {
    /// DID URI (e.g., "did:ion:123...")
    pub id: String,
    /// DID Document
    pub document: DIDDocument,
    /// Private keys associated with this DID
    #[serde(skip_serializing)]
    pub private_keys: HashMap<String, Vec<u8>>,
}

/// DID Document
///
/// The public representation of a DID, containing verification methods
/// and service endpoints as defined in the DID Core specification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DIDDocument {
    /// DID context
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// DID URI
    pub id: String,
    /// Verification methods
    #[serde(default)]
    pub verification_method: Vec<VerificationMethod>,
    /// Authentication methods
    #[serde(default)]
    pub authentication: Vec<String>,
    /// Assertion methods
    #[serde(default)]
    pub assertion_method: Vec<String>,
    /// Service endpoints
    #[serde(default)]
    pub service: Vec<Service>,
}

/// Verification Method
///
/// A cryptographic mechanism used for authentication and
/// digital signatures within a DID.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationMethod {
    /// ID of the verification method
    pub id: String,
    /// Type of the verification method
    #[serde(rename = "type")]
    pub vm_type: String,
    /// Controller of the verification method
    pub controller: String,
    /// Public key in JWK format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_jwk: Option<JWK>,
}

/// JSON Web Key
///
/// A cryptographic key representation in JSON format.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JWK {
    /// Key type
    pub kty: String,
    /// Curve (for EC keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    /// X coordinate (for EC keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    /// Y coordinate (for EC keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    /// Key ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

/// Service
///
/// A service endpoint for a DID.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// ID of the service
    pub id: String,
    /// Type of the service
    #[serde(rename = "type")]
    pub service_type: String,
    /// Service endpoint URL
    pub service_endpoint: String,
}

impl DIDManager {
    /// Create a new DID manager with the specified method
    pub fn new(method: &str) -> Self {
        Self {
            dids: Arc::new(Mutex::new(HashMap::new())),
            default_did: None,
            method: method.to_string(),
        }
    }

    /// Create a new DID with the configured method
    pub fn create_did(&self) -> Web5Result<DID> {
        // Generate a random ID for the DID
        let id = format!("did:{}:{}", self.method, generate_random_id());

        // Generate a key pair for this DID
        let private_key = generate_private_key();
        let public_key_jwk = generate_public_key_jwk(&private_key);

        // Create verification method
        let verification_method = VerificationMethod {
            id: format!("{}#key-1", id),
            vm_type: "JsonWebKey2020".to_string(),
            controller: id.clone(),
            public_key_jwk: Some(public_key_jwk),
        };

        // Create a basic DID document
        let document = DIDDocument {
            context: vec!["https://www.w3.org/ns/did/v1".to_string()],
            id: id.clone(),
            verification_method: vec![verification_method],
            authentication: vec![format!("{}#key-1", id)],
            assertion_method: vec![format!("{}#key-1", id)],
            service: Vec::new(),
        };

        // Create the DID with private keys
        let mut private_keys = HashMap::new();
        private_keys.insert("key-1".to_string(), private_key);

        let did = DID {
            id: id.clone(),
            document,
            private_keys,
        };

        // Store the DID
        {
            let mut dids = self
                .dids
                .lock()
                .map_err(|e| format!("Mutex lock error: {}", e))?;
            dids.insert(id.clone(), did.clone());
        }

        Ok(did)
    }

    /// Resolve a DID to its document
    pub fn resolve_did(&self, did: &str) -> Result<DIDDocument, Box<dyn Error>> {
        // First, check if we have the DID locally
        let dids = self
            .dids
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        if let Some(did_obj) = dids.get(did) {
            return Ok(did_obj.document.clone());
        }

        // If not found locally, return an error (future: implement remote resolution)
        Err(format!("DID not found: {}", did).into())
    }

    /// Set the default DID
    pub fn set_default_did(&mut self, did: &str) -> Result<(), Box<dyn Error>> {
        let dids = self
            .dids
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        if dids.contains_key(did) {
            self.default_did = Some(did.to_string());
            Ok(())
        } else {
            Err(format!("DID {} not found", did).into())
        }
    }

    /// Get the default DID
    pub fn get_default_did(&self) -> Result<Option<String>, Box<dyn Error>> {
        Ok(self.default_did.clone())
    }

    /// Sign data with a DID's private key
    pub fn sign(&self, did: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Get the DID
        let dids = self
            .dids
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        let did_obj = dids.get(did).ok_or_else(|| format!("DID not found: {}", did))?;

        // Get the first private key for signing
        if let Some((_, private_key_bytes)) = did_obj.private_keys.iter().next() {
            // Parse the private key
            let private_key = secp256k1::SecretKey::from_slice(private_key_bytes)
                .map_err(|e| format!("Invalid private key: {}", e))?;
            
            // Create secp256k1 context
            let secp = secp256k1::Secp256k1::signing_only();
            
            // Hash the data (using SHA256)
            let hash = {
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize()
            };
            
            // Create message from hash
            let message = secp256k1::Message::from_slice(&hash)
                .map_err(|e| format!("Failed to create message: {}", e))?;
            
            // Sign the message
            let signature = secp.sign_ecdsa(&message, &private_key);
            
            // Return the signature bytes
            Ok(signature.serialize_compact().to_vec())
        } else {
            Err("No private keys found for DID".into())
        }
    }

    /// Get a list of all DIDs
    pub fn dids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let dids = self
            .dids
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        Ok(dids.keys().cloned().collect())
    }

    /// Get a DID by ID
    pub fn get_did(&self, did_id: &str) -> Web5Result<Option<DID>> {
        let dids = self
            .dids
            .lock()
            .map_err(|e| Web5Error::Storage(format!("Mutex lock error: {}", e)))?;
        Ok(dids.get(did_id).cloned())
    }

    /// List all DIDs
    pub fn list_dids(&self) -> Vec<DID> {
        let dids = self
            .dids
            .lock()
            .unwrap_or_else(|_| panic!("Failed to lock mutex"));
        dids.values().cloned().collect()
    }
}

/// Identity manager for Web5 DID operations
#[derive(Debug, Clone)]
pub struct IdentityManager {
    did_manager: DIDManager,
}

impl IdentityManager {
    pub fn new(namespace: &str) -> Self {
        Self {
            did_manager: DIDManager::new(namespace),
        }
    }

    pub fn create_identity(&mut self) -> Web5Result<DID> {
        self.did_manager.create_did()
    }

    pub fn get_identity(&self, did_id: &str) -> Web5Result<Option<DID>> {
        self.did_manager.get_did(did_id)
    }

    pub fn list_identities(&self) -> Vec<DID> {
        self.did_manager.list_dids()
    }
}

/// Generate a random ID for a DID
/// [AIS-3] Properly handles errors without using ? operator
fn generate_random_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("{:x}", now)
}

/// Generate a private key for cryptographic operations
fn generate_private_key() -> Vec<u8> {
    // Generate a 32-byte private key (simplified implementation)
    use rand::RngCore;
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

/// Generate a public key JWK from a private key
fn generate_public_key_jwk(private_key: &[u8]) -> JWK {
    // Simplified implementation - in production this would derive the actual public key
    // from the private key using proper cryptographic operations
    use base64::Engine;
    
    // For demonstration, we'll create a placeholder JWK
    JWK {
        kty: "EC".to_string(),
        crv: Some("secp256k1".to_string()),
        x: Some(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&private_key[..16])),
        y: Some(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&private_key[16..])),
        kid: Some("key-1".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_did() -> Result<(), Box<dyn Error>> {
        let manager = DIDManager::new("example");
        let did = manager.create_did()?;
        assert!(!did.id.is_empty());
        assert_eq!(did.id.starts_with("did:example:"), true);
        assert!(!did.private_keys.is_empty());
        Ok(())
    }

    #[test]
    fn test_default_did() -> Result<(), Box<dyn Error>> {
        let mut manager = DIDManager::new("example");
        let did = manager.create_did()?;

        // Initially no default DID
        assert!(manager.get_default_did()?.is_none());

        // Set and get default DID
        manager.set_default_did(&did.id)?;
        assert_eq!(manager.get_default_did()?.unwrap(), did.id);
        Ok(())
    }
}
