// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Verifiable Credentials implementation for Web5
//!
//! This module provides functionality for creating, validating, and managing
//! Verifiable Credentials following the W3C Verifiable Credentials Data Model.
//! It integrates with Bitcoin for cryptographic security and credential anchoring.

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use bitcoin::Network;
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use sha2::{Sha256, Digest};
use tracing::{debug, error, info, warn};

use super::did::{DidManager, DidDocument};

/// Verifiable Credential as per W3C specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    /// Context definitions
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    
    /// Credential ID
    pub id: String,
    
    /// Credential types
    #[serde(rename = "type")]
    pub types: Vec<String>,
    
    /// Credential issuer
    pub issuer: String,
    
    /// Issuance date
    #[serde(rename = "issuanceDate")]
    pub issuance_date: DateTime<Utc>,
    
    /// Expiration date
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,
    
    /// Credential subject
    pub credentialSubject: CredentialSubject,
    
    /// Credential status
    #[serde(rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<CredentialStatus>,
    
    /// Credential proof
    pub proof: Option<Proof>,
}

/// Credential subject containing claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    /// Subject identifier (DID)
    pub id: String,
    
    /// Claims about the subject
    #[serde(flatten)]
    pub claims: HashMap<String, Value>,
}

/// Credential status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialStatus {
    /// Status identifier
    pub id: String,
    
    /// Status type
    #[serde(rename = "type")]
    pub type_: String,
    
    /// Status properties
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

/// Credential proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    /// Proof type
    #[serde(rename = "type")]
    pub type_: String,
    
    /// Creation date
    pub created: DateTime<Utc>,
    
    /// Verification method
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
    
    /// Proof purpose
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    
    /// Proof value (signature)
    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

/// Verifiable Presentation as per W3C specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiablePresentation {
    /// Context definitions
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    
    /// Presentation ID
    pub id: String,
    
    /// Presentation types
    #[serde(rename = "type")]
    pub types: Vec<String>,
    
    /// Holder of the presentation
    pub holder: String,
    
    /// Verifiable credentials included in the presentation
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<VerifiableCredential>,
    
    /// Presentation proof
    pub proof: Option<Proof>,
}

/// Credential Manager for issuing and verifying credentials
pub struct CredentialManager {
    /// DID Manager for identity operations
    did_manager: Arc<DidManager>,
}

impl CredentialManager {
    /// Create a new Credential Manager
    pub fn new(did_manager: Arc<DidManager>) -> Self {
        Self {
            did_manager,
        }
    }
    
    /// Issue a new verifiable credential
    pub async fn issue_credential(
        &self,
        issuer_did: &str,
        subject_did: &str,
        credential_type: &str,
        claims: HashMap<String, Value>,
        valid_for_days: Option<u32>,
    ) -> Result<VerifiableCredential> {
        // Verify that the issuer DID exists
        let issuer_did_doc = self.did_manager.resolve_did(issuer_did).await?;
        
        // Create a credential ID
        let mut hasher = Sha256::new();
        hasher.update(issuer_did.as_bytes());
        hasher.update(subject_did.as_bytes());
        hasher.update(credential_type.as_bytes());
        let hash = hasher.finalize();
        let credential_id = format!("urn:vc:{}", base64::encode_config(&hash[0..16], base64::URL_SAFE_NO_PAD));
        
        // Set issuance and expiration dates
        let issuance_date = Utc::now();
        let expiration_date = valid_for_days.map(|days| {
            issuance_date + Duration::days(days as i64)
        });
        
        // Create the credential
        let credential = VerifiableCredential {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
                format!("https://www.w3.org/2018/credentials/{}/v1", credential_type.to_lowercase()),
            ],
            id: credential_id,
            types: vec!["VerifiableCredential".to_string(), credential_type.to_string()],
            issuer: issuer_did.to_string(),
            issuance_date,
            expiration_date,
            credentialSubject: CredentialSubject {
                id: subject_did.to_string(),
                claims,
            },
            credential_status: Some(CredentialStatus {
                id: format!("https://credential-status.example.com/status/{}", credential_type),
                type_: "RevocationList2020".to_string(),
                properties: HashMap::new(),
            }),
            proof: None,
        };
        
        // Serialize the credential for signing (excluding the proof)
        let credential_json = serde_json::to_string(&credential)?;
        
        // Sign the credential
        // TODO: Implement proper signing
        let signature = "todo_implement_proper_signing".to_string();
        
        // Create the proof
        let proof = Proof {
            type_: "Ed25519Signature2020".to_string(),
            created: Utc::now(),
            verification_method: format!("{}#key-1", issuer_did),
            proof_purpose: "assertionMethod".to_string(),
            proof_value: signature,
        };
        
        // Add the proof to the credential
        let mut signed_credential = credential;
        signed_credential.proof = Some(proof);
        
        Ok(signed_credential)
    }
    
    /// Verify a credential
    pub async fn verify_credential(&self, credential: &VerifiableCredential) -> Result<bool> {
        // Check if the credential has a proof
        let proof = credential.proof.as_ref().ok_or_else(|| anyhow!("Credential has no proof"))?;
        
        // Check if the credential has expired
        if let Some(expiration_date) = credential.expiration_date {
            if expiration_date < Utc::now() {
                return Ok(false);
            }
        }
        
        // Resolve the issuer's DID
        let issuer_did_doc = self.did_manager.resolve_did(&credential.issuer).await?;
        
        // Verify the proof
        // TODO: Implement proper verification
        // For now, just return true
        Ok(true)
    }
    
    /// Create a verifiable presentation
    pub async fn create_presentation(
        &self,
        holder_did: &str,
        credentials: Vec<VerifiableCredential>,
    ) -> Result<VerifiablePresentation> {
        // Verify that the holder DID exists
        let holder_did_doc = self.did_manager.resolve_did(holder_did).await?;
        
        // Create a presentation ID
        let mut hasher = Sha256::new();
        hasher.update(holder_did.as_bytes());
        for credential in &credentials {
            hasher.update(credential.id.as_bytes());
        }
        let hash = hasher.finalize();
        let presentation_id = format!("urn:vp:{}", base64::encode_config(&hash[0..16], base64::URL_SAFE_NO_PAD));
        
        // Create the presentation
        let presentation = VerifiablePresentation {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
            ],
            id: presentation_id,
            types: vec!["VerifiablePresentation".to_string()],
            holder: holder_did.to_string(),
            verifiable_credential: credentials,
            proof: None,
        };
        
        // Serialize the presentation for signing (excluding the proof)
        let presentation_json = serde_json::to_string(&presentation)?;
        
        // Sign the presentation
        // TODO: Implement proper signing
        let signature = "todo_implement_proper_signing".to_string();
        
        // Create the proof
        let proof = Proof {
            type_: "Ed25519Signature2020".to_string(),
            created: Utc::now(),
            verification_method: format!("{}#key-1", holder_did),
            proof_purpose: "authentication".to_string(),
            proof_value: signature,
        };
        
        // Add the proof to the presentation
        let mut signed_presentation = presentation;
        signed_presentation.proof = Some(proof);
        
        Ok(signed_presentation)
    }
    
    /// Verify a presentation
    pub async fn verify_presentation(&self, presentation: &VerifiablePresentation) -> Result<bool> {
        // Check if the presentation has a proof
        let proof = presentation.proof.as_ref().ok_or_else(|| anyhow!("Presentation has no proof"))?;
        
        // Resolve the holder's DID
        let holder_did_doc = self.did_manager.resolve_did(&presentation.holder).await?;
        
        // Verify the presentation proof
        // TODO: Implement proper verification
        
        // Verify each credential in the presentation
        for credential in &presentation.verifiable_credential {
            if !self.verify_credential(credential).await? {
                return Ok(false);
            }
        }
        
        // All verifications passed
        Ok(true)
    }
    
    /// Revoke a credential
    pub async fn revoke_credential(&self, credential_id: &str, issuer_did: &str) -> Result<()> {
        // TODO: Implement credential revocation
        Err(anyhow!("Credential revocation not yet implemented"))
    }
}

/// Tests for Verifiable Credentials functionality
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::wallet::{BitcoinWallet, WalletConfig};
    
    #[tokio::test]
    async fn test_credential_issuance() {
        // Create test wallet
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("wallet.db");
        
        let config = WalletConfig {
            name: "test_wallet".to_string(),
            database_path: db_path,
            network: Network::Testnet,
            electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
            password: None,
            mnemonic: None,
            use_taproot: true,
        };
        
        let wallet = BitcoinWallet::new(config).await.unwrap();
        let wallet_arc = Arc::new(wallet);
        
        // Create DID manager
        let did_manager = Arc::new(DidManager::new(wallet_arc, Network::Testnet));
        
        // Create DIDs for issuer and subject
        let issuer_did_doc = did_manager.create_did().await.unwrap();
        let subject_did_doc = did_manager.create_did().await.unwrap();
        
        // Create Credential manager
        let credential_manager = CredentialManager::new(did_manager.clone());
        
        // Create claims
        let mut claims = HashMap::new();
        claims.insert("name".to_string(), Value::String("Alice".to_string()));
        claims.insert("email".to_string(), Value::String("alice@example.com".to_string()));
        
        // Issue a credential
        let credential = credential_manager.issue_credential(
            &issuer_did_doc.id,
            &subject_did_doc.id,
            "EmailCredential",
            claims,
            Some(365),
        ).await.unwrap();
        
        // Verify the credential structure
        assert_eq!(credential.issuer, issuer_did_doc.id);
        assert_eq!(credential.credentialSubject.id, subject_did_doc.id);
        assert!(credential.types.contains(&"EmailCredential".to_string()));
        
        // Verify the credential
        let is_valid = credential_manager.verify_credential(&credential).await.unwrap();
        assert!(is_valid);
        
        // Create a presentation
        let presentation = credential_manager.create_presentation(
            &subject_did_doc.id,
            vec![credential],
        ).await.unwrap();
        
        // Verify the presentation
        let is_valid = credential_manager.verify_presentation(&presentation).await.unwrap();
        assert!(is_valid);
    }
}
