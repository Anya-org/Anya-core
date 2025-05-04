use async_trait::async_trait;
use bitcoin::psbt::{Input, Output, Psbt};
use bitcoin::{Address, Network, Script, ScriptBuf, Transaction};
use chrono::Utc;
use rand::prelude::*;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::security::hsm::config::SoftHsmConfig;
use crate::security::hsm::error::HsmError;
use crate::security::hsm::provider::{
    EncryptionAlgorithm, HsmOperation, HsmProvider, HsmProviderStatus, HsmRequest, HsmResponse,
    KeyGenParams, KeyInfo, KeyPair, KeyType, KeyUsage, SigningAlgorithm,
};

/// Software-based HSM provider implementation for development and testing
/// Uses in-memory key storage for Bitcoin testnet operations
#[derive(Debug)]
pub struct SoftwareHsmProvider {
    config: SoftHsmConfig,
    keys: Mutex<HashMap<String, KeyInfo>>,
    key_data: Mutex<HashMap<String, Vec<u8>>>,
    network: Network,
    secp: Secp256k1<secp256k1::All>,
}

impl SoftwareHsmProvider {
    /// Create a new software HSM provider
    pub fn new(config: &SoftHsmConfig) -> Result<Self, HsmError> {
        Ok(Self {
            config: config.clone(),
            keys: Mutex::new(HashMap::new()),
            key_data: Mutex::new(HashMap::new()),
            network: Network::Testnet, // Always use testnet for testing
            secp: Secp256k1::new(),
        })
    }

    /// Generate a random key ID
    fn generate_key_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Generate new Bitcoin key for testnet
    async fn generate_bitcoin_key(
        &self,
        params: &KeyGenParams,
    ) -> Result<(Vec<u8>, Vec<u8>), HsmError> {
        let mut rng = thread_rng();
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&self.secp, &secret_key);

        // Generate testnet address
        let address = Address::p2wpkh(&public_key, self.network).map_err(|e| {
            HsmError::KeyGenerationError(format!("Failed to create testnet address: {}", e))
        })?;

        tracing::info!("Generated new testnet address: {}", address);

        Ok((
            public_key.serialize().to_vec(),
            secret_key.secret_bytes().to_vec(),
        ))
    }

    /// Sign a Bitcoin transaction for testnet
    async fn sign_bitcoin_transaction(&self, key_id: &str, tx: &mut Psbt) -> Result<(), HsmError> {
        let key_data = self.key_data.lock().await;
        let secret_data = key_data
            .get(key_id)
            .ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;

        let secret_key = SecretKey::from_slice(secret_data)
            .map_err(|e| HsmError::SigningError(format!("Invalid key data: {}", e)))?;

        // Sign the transaction for testnet
        let success = tx.sign(&secret_key, &self.secp);
        if !success {
            return Err(HsmError::SigningError(
                "Failed to sign transaction".to_string(),
            ));
        }

        tracing::info!("Successfully signed testnet transaction");
        Ok(())
    }
}

#[async_trait]
impl HsmProvider for SoftwareHsmProvider {
    async fn initialize(&self) -> Result<(), HsmError> {
        // For software HSM, initialization just ensures the token directory exists
        std::fs::create_dir_all(&self.config.token_dir).map_err(|e| {
            HsmError::InitializationError(format!("Failed to create token directory: {}", e))
        })?;

        tracing::info!("Initialized software HSM for Bitcoin testnet operations");
        Ok(())
    }

    async fn generate_key(&self, params: KeyGenParams) -> Result<KeyPair, HsmError> {
        let key_id = params.id.unwrap_or_else(|| self.generate_key_id());

        // Generate key pair based on key type with real testnet support
        let (public_key, private_key) = match &params.key_type {
            KeyType::Ec { curve }
                if *curve == crate::security::hsm::provider::EcCurve::Secp256k1 =>
            {
                self.generate_bitcoin_key(&params).await?
            }
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

    async fn sign(
        &self,
        key_id: &str,
        algorithm: SigningAlgorithm,
        data: &[u8],
    ) -> Result<Vec<u8>, HsmError> {
        // Check if key exists and has signing capability
        let keys = self.keys.lock().await;
        let key_info = keys
            .get(key_id)
            .ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;

        if !key_info.usages.contains(&KeyUsage::Sign) {
            return Err(HsmError::PermissionDenied(
                "Key does not have signing permission".to_string(),
            ));
        }

        // Get the private key
        let key_data = self.key_data.lock().await;
        let private_key_data = key_data
            .get(key_id)
            .ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;

        // Perform real signing based on algorithm
        match algorithm {
            SigningAlgorithm::EcdsaSha256 => {
                let secret_key = SecretKey::from_slice(private_key_data)
                    .map_err(|e| HsmError::SigningError(format!("Invalid key data: {}", e)))?;

                // Create message hash
                let mut hasher = Sha256::new();
                hasher.update(data);
                let message_hash = hasher.finalize();

                // Sign the hash
                let message = secp256k1::Message::from_slice(&message_hash)
                    .map_err(|e| HsmError::SigningError(format!("Invalid message hash: {}", e)))?;

                let signature = self.secp.sign_ecdsa(&message, &secret_key);
                Ok(signature.serialize_der().to_vec())
            }
            _ => Err(HsmError::UnsupportedOperation(format!(
                "Unsupported signing algorithm: {:?}",
                algorithm
            ))),
        }
    }

    async fn verify(
        &self,
        key_id: &str,
        algorithm: SigningAlgorithm,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, HsmError> {
        // Check if key exists and has verify capability
        let keys = self.keys.lock().await;
        let key_info = keys
            .get(key_id)
            .ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;

        if !key_info.usages.contains(&KeyUsage::Verify) {
            return Err(HsmError::PermissionDenied(
                "Key does not have verify permission".to_string(),
            ));
        }

        // Real verification for testnet
        match algorithm {
            SigningAlgorithm::EcdsaSha256 => {
                // Get the public key
                match key_info.key_type {
                    KeyType::Ec { curve }
                        if curve == crate::security::hsm::provider::EcCurve::Secp256k1 =>
                    {
                        // Create message hash
                        let mut hasher = Sha256::new();
                        hasher.update(data);
                        let message_hash = hasher.finalize();

                        // Get the serialized public key from key storage or regenerate
                        let pubkey_bytes = self.export_public_key(key_id).await?;
                        let public_key = PublicKey::from_slice(&pubkey_bytes).map_err(|e| {
                            HsmError::VerificationError(format!("Invalid public key data: {}", e))
                        })?;

                        // Parse the signature
                        let sig =
                            secp256k1::ecdsa::Signature::from_der(signature).map_err(|e| {
                                HsmError::VerificationError(format!(
                                    "Invalid signature format: {}",
                                    e
                                ))
                            })?;

                        // Verify
                        let message =
                            secp256k1::Message::from_slice(&message_hash).map_err(|e| {
                                HsmError::VerificationError(format!("Invalid message hash: {}", e))
                            })?;

                        match self.secp.verify_ecdsa(&message, &sig, &public_key) {
                            Ok(_) => Ok(true),
                            Err(_) => Ok(false),
                        }
                    }
                    _ => Err(HsmError::UnsupportedKeyType),
                }
            }
            _ => Err(HsmError::UnsupportedOperation(format!(
                "Unsupported verification algorithm: {:?}",
                algorithm
            ))),
        }
    }

    async fn export_public_key(&self, key_id: &str) -> Result<Vec<u8>, HsmError> {
        // Check if key exists
        let keys = self.keys.lock().await;
        let key_info = keys
            .get(key_id)
            .ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;

        // Get the private key and derive public key
        let key_data = self.key_data.lock().await;
        let private_key_data = key_data
            .get(key_id)
            .ok_or_else(|| HsmError::KeyNotFound(key_id.to_string()))?;

        match key_info.key_type {
            KeyType::Ec { curve }
                if curve == crate::security::hsm::provider::EcCurve::Secp256k1 =>
            {
                let secret_key = SecretKey::from_slice(private_key_data).map_err(|e| {
                    HsmError::KeyGenerationError(format!("Invalid key data: {}", e))
                })?;
                let public_key = PublicKey::from_secret_key(&self.secp, &secret_key);
                Ok(public_key.serialize().to_vec())
            }
            _ => Err(HsmError::UnsupportedKeyType),
        }
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
                    .map_err(|e| {
                        HsmError::InvalidParameters(format!(
                            "Invalid key generation parameters: {}",
                            e
                        ))
                    })?;

                let key_pair = self.generate_key(params).await?;
                let response_data = serde_json::to_value(key_pair)
                    .map_err(|e| HsmError::SerializationError(e.to_string()))?;

                Ok(HsmResponse::success(request.id, Some(response_data)))
            }
            HsmOperation::Sign => {
                let params: SignParams = serde_json::from_value(request.parameters.clone())
                    .map_err(|e| {
                        HsmError::InvalidParameters(format!("Invalid signing parameters: {}", e))
                    })?;

                let signature = self
                    .sign(&params.key_id, params.algorithm, &params.data)
                    .await?;
                let response_data = serde_json::to_value(Base64SignatureResponse {
                    signature: base64::encode(&signature),
                    algorithm: params.algorithm,
                })
                .map_err(|e| HsmError::SerializationError(e.to_string()))?;

                Ok(HsmResponse::success(request.id, Some(response_data)))
            }
            HsmOperation::Custom(op) if op == "sign_bitcoin_tx" => {
                let params: BitcoinTxSignParams =
                    serde_json::from_value(request.parameters.clone()).map_err(|e| {
                        HsmError::InvalidParameters(format!(
                            "Invalid Bitcoin TX signing parameters: {}",
                            e
                        ))
                    })?;

                // Decode PSBT
                let mut psbt = bitcoin::psbt::Psbt::from_str(&params.psbt)
                    .map_err(|e| HsmError::InvalidParameters(format!("Invalid PSBT: {}", e)))?;

                // Sign the transaction
                self.sign_bitcoin_transaction(&params.key_id, &mut psbt)
                    .await?;

                // Return the signed PSBT
                let response_data = serde_json::json!({
                    "signed_psbt": psbt.to_string(),
                    "network": "testnet",
                });

                Ok(HsmResponse::success(request.id, Some(response_data)))
            }
            _ => Err(HsmError::UnsupportedOperation(format!(
                "{:?}",
                request.operation
            ))),
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

/// Parameters for Bitcoin transaction signing
#[derive(Debug, serde::Deserialize)]
struct BitcoinTxSignParams {
    key_id: String,
    psbt: String,
}

/// Response for signature in base64 format
#[derive(Debug, serde::Serialize)]
struct Base64SignatureResponse {
    signature: String,
    algorithm: SigningAlgorithm,
}
