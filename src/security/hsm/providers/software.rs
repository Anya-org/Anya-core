use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;
use rand::prelude::*;
use chrono::Utc;

// Added comment to ensure file is saved
use crate::security::hsm::config::SoftHsmConfig;
use crate::security::hsm::provider::{
    HsmProvider, HsmProviderStatus, KeyType, KeyInfo, KeyGenParams,
    KeyPair, KeyUsage, SigningAlgorithm, EncryptionAlgorithm,
    HsmRequest, HsmResponse, HsmOperation
};
use crate::security::hsm::error::HsmError;

/// Software-based HSM provider implementation for development and testing
#[derive(Debug)]
pub struct SoftwareHsmProvider {
    config: SoftHsmConfig,
    keys: Mutex<HashMap<String, KeyInfo>>,
    key_data: Mutex<HashMap<String, Vec<u8>>>, 
}

impl SoftwareHsmProvider {
    /// Create a new software HSM provider
    pub fn new(config: &SoftHsmConfig) -> Result<Self, HsmError> {
        Ok(Self {
            config: config.clone(),
            keys: Mutex::new(HashMap::new()),
            key_data: Mutex::new(HashMap::new()),
        })
    }

    /// Generate a random key ID
    fn generate_key_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

#[async_trait]
impl HsmProvider for SoftwareHsmProvider {
    async fn initialize(&self) -> Result<(), HsmError> {
        // For software HSM, initialization just ensures the token directory exists
        std::fs::create_dir_all(&self.config.token_dir)
            .map_err(|e| HsmError::InitializationError(format!("Failed to create token directory: {}", e)))?;
        Ok(())
    }
    
    async fn generate_key(&self, params: KeyGenParams) -> Result<KeyPair, HsmError> {
        let key_id = params.id.unwrap_or_else(|| self.generate_key_id());
        
        // Generate key pair based on key type
        let (public_key, private_key) = match &params.key_type {
            KeyType::Rsa { bits } => {
                // Simulate RSA key generation
                let pub_key = vec![1, 2, 3, 4]; // Placeholder
                let priv_key = vec![5, 6, 7, 8]; // Placeholder
                (pub_key, priv_key)
            },
            KeyType::Ec { curve } => {
                // Simulate EC key generation
                let pub_key = vec![1, 2, 3, 4]; // Placeholder
                let priv_key = vec![5, 6, 7, 8]; // Placeholder
                (pub_key, priv_key)
            },
            KeyType::Ed25519 => {
                // Simulate Ed25519 key generation
                let pub_key = vec![1, 2, 3, 4]; // Placeholder
                let priv_key = vec![5, 6, 7, 8]; // Placeholder
                (pub_key, priv_key)
            },
            KeyType::X25519 => {
                // Simulate X25519 key generation
                let pub_key = vec![1, 2, 3, 4]; // Placeholder
                let priv_key = vec![5, 6, 7, 8]; // Placeholder
                (pub_key, priv_key)
            },
            _ => return Err(HsmError::UnsupportedKeyType),
        };
        
        // Store key information
        let key_info = KeyInfo {
            id: key_id.clone(),
            label: params.label.clone(),
            key_type: params.key_type.clone(),
            extractable: params.extractable,
            usages: params.usages.clone(),
            created_at: Utc::now(),
            expires_at: params.expires_at,
            attributes: params.attributes.clone(),
        };
        
        let mut keys = self.keys.lock().await;
        let mut key_data = self.key_data.lock().await;
        
        keys.insert(key_id.clone(), key_info);
        key_data.insert(key_id.clone(), private_key);
        
        Ok(KeyPair {
            id: key_id,
            key_type: params.key_type,
            public_key,
            private_key_handle: key_id.clone(),
        })
    }
    
    async fn sign(&self, key_id: &str, algorithm: SigningAlgorithm, data: &[u8]) -> Result<Vec<u8>, HsmError> {
        // Check if key exists and has signing capability
        let keys = self.keys.lock().await;
        let key_info = keys.get(key_id).ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;
        
        if !key_info.usages.contains(&KeyUsage::Sign) {
            return Err(HsmError::PermissionDenied("Key does not have signing permission".to_string()));
        }
        
        // Simulate signing
        let mut rng = thread_rng();
        let mut signature = vec![0u8; 64]; // Fixed size signature for simplicity
        rng.fill_bytes(&mut signature);
        
        Ok(signature)
    }
    
    async fn verify(&self, key_id: &str, algorithm: SigningAlgorithm, data: &[u8], signature: &[u8]) -> Result<bool, HsmError> {
        // Check if key exists and has verify capability
        let keys = self.keys.lock().await;
        let key_info = keys.get(key_id).ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;
        
        if !key_info.usages.contains(&KeyUsage::Verify) {
            return Err(HsmError::PermissionDenied("Key does not have verify permission".to_string()));
        }
        
        // Simulate verification (always returns true for testing)
        Ok(true)
    }
    
    async fn encrypt(&self, key_id: &str, algorithm: EncryptionAlgorithm, data: &[u8]) -> Result<Vec<u8>, HsmError> {
        // Check if key exists and has encryption capability
        let keys = self.keys.lock().await;
        let key_info = keys.get(key_id).ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;
        
        if !key_info.usages.contains(&KeyUsage::Encrypt) {
            return Err(HsmError::PermissionDenied("Key does not have encryption permission".to_string()));
        }
        
        // Simulate encryption (simple XOR with fixed key for demo)
        let key_byte = 0x42; // Fixed key byte
        let encrypted = data.iter().map(|b| b ^ key_byte).collect();
        
        Ok(encrypted)
    }
    
    async fn decrypt(&self, key_id: &str, algorithm: EncryptionAlgorithm, encrypted_data: &[u8]) -> Result<Vec<u8>, HsmError> {
        // Check if key exists and has decryption capability
        let keys = self.keys.lock().await;
        let key_info = keys.get(key_id).ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;
        
        if !key_info.usages.contains(&KeyUsage::Decrypt) {
            return Err(HsmError::PermissionDenied("Key does not have decryption permission".to_string()));
        }
        
        // Simulate decryption (simple XOR with fixed key for demo)
        let key_byte = 0x42; // Fixed key byte
        let decrypted = encrypted_data.iter().map(|b| b ^ key_byte).collect();
        
        Ok(decrypted)
    }
    
    async fn export_public_key(&self, key_id: &str) -> Result<Vec<u8>, HsmError> {
        // Check if key exists
        let keys = self.keys.lock().await;
        let _key_info = keys.get(key_id).ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;
        
        // Return dummy public key
        Ok(vec![1, 2, 3, 4])
    }
    
    async fn list_keys(&self) -> Result<Vec<KeyInfo>, HsmError> {
        let keys = self.keys.lock().await;
        Ok(keys.values().cloned().collect())
    }
    
    async fn delete_key(&self, key_id: &str) -> Result<(), HsmError> {
        let mut keys = self.keys.lock().await;
        let mut key_data = self.key_data.lock().await;
        
        if keys.remove(key_id).is_none() {
            return Err(HsmError::KeyNotFound(key_id.to_string()));
        }
        
        key_data.remove(key_id);
        Ok(())
    }
    
    async fn get_status(&self) -> Result<HsmProviderStatus, HsmError> {
        Ok(HsmProviderStatus::Ready)
    }
    
    async fn close(&self) -> Result<(), HsmError> {
        Ok(())
    }
    
    async fn execute_operation(&self, request: HsmRequest) -> Result<HsmResponse, HsmError> {
        match request.operation {
            HsmOperation::GenerateKey => {
                let params: KeyGenParams = serde_json::from_value(request.parameters.clone())
                    .map_err(|e| HsmError::InvalidParameters(format!("Invalid key generation parameters: {}", e)))?;
                
                let key_pair = self.generate_key(params).await?;
                let response_data = serde_json::to_value(key_pair)
                    .map_err(|e| HsmError::SerializationError(e.to_string()))?;
                
                Ok(HsmResponse::success(request.id, Some(response_data)))
            },
            HsmOperation::Sign => {
                let params: SignParams = serde_json::from_value(request.parameters.clone())
                    .map_err(|e| HsmError::InvalidParameters(format!("Invalid signing parameters: {}", e)))?;
                
                let signature = self.sign(&params.key_id, params.algorithm, &params.data).await?;
                let response_data = serde_json::to_value(signature)
                    .map_err(|e| HsmError::SerializationError(e.to_string()))?;
                
                Ok(HsmResponse::success(request.id, Some(response_data)))
            },
            // Implement other operations similarly
            _ => Err(HsmError::UnsupportedOperation(format!("{:?}", request.operation))),
        }
    }
}

/// Parameters for signing operation
#[derive(Debug, serde::Deserialize)]
struct SignParams {
    key_id: String,
    algorithm: SigningAlgorithm,
    data: Vec<u8>,
} 