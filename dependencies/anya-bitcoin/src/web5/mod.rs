// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Web5 implementation for decentralized identity and data
//!
//! Web5 is a decentralized web platform that combines decentralized identifiers,
//! verifiable credentials, and decentralized web nodes to return ownership and
//! control over identity and data to individuals. This module provides implementations
//! of the core Web5 components built on top of Bitcoin.
//!
//! Key features:
//! - Decentralized Identifiers (DIDs) for self-sovereign identity
//! - Verifiable Credentials for privacy-preserving attestations
//! - Decentralized Web Nodes (DWNs) for personal data storage
//! - Handshake integration for decentralized DNS

pub mod did;
pub mod credential;
pub mod dwn;
pub mod handshake;

use anyhow::Result;
use std::sync::Arc;
use bitcoin::Network;

use crate::wallet::BitcoinWallet;
use did::{DidManager, DidDocument};
use credential::{CredentialManager, VerifiableCredential};

/// Web5 manager for coordinating all Web5 components
pub struct Web5Manager {
    /// DID manager for identity operations
    did_manager: Arc<DidManager>,
    
    /// Credential manager for verifiable credentials
    credential_manager: Arc<CredentialManager>,
    
    /// Bitcoin network
    network: Network,
}

impl Web5Manager {
    /// Create a new Web5 manager
    pub fn new(wallet: Arc<BitcoinWallet>, network: Network) -> Self {
        let did_manager = Arc::new(DidManager::new(wallet, network));
        let credential_manager = Arc::new(CredentialManager::new(did_manager.clone()));
        
        Self {
            did_manager,
            credential_manager,
            network,
        }
    }
    
    /// Create a new decentralized identifier
    pub async fn create_did(&self) -> Result<DidDocument> {
        self.did_manager.create_did().await
    }
    
    /// Resolve a decentralized identifier
    pub async fn resolve_did(&self, did: &str) -> Result<did::DidResolutionResult> {
        self.did_manager.resolve_did(did).await
    }
    
    /// Issue a verifiable credential
    pub async fn issue_credential(
        &self,
        issuer_did: &str,
        subject_did: &str,
        credential_type: &str,
        claims: std::collections::HashMap<String, serde_json::Value>,
        valid_for_days: Option<u32>,
    ) -> Result<VerifiableCredential> {
        self.credential_manager.issue_credential(
            issuer_did,
            subject_did,
            credential_type,
            claims,
            valid_for_days,
        ).await
    }
    
    /// Verify a credential
    pub async fn verify_credential(&self, credential: &VerifiableCredential) -> Result<bool> {
        self.credential_manager.verify_credential(credential).await
    }
    
    /// Create a verifiable presentation
    pub async fn create_presentation(
        &self,
        holder_did: &str,
        credentials: Vec<VerifiableCredential>,
    ) -> Result<credential::VerifiablePresentation> {
        self.credential_manager.create_presentation(holder_did, credentials).await
    }
    
    /// Verify a presentation
    pub async fn verify_presentation(&self, presentation: &credential::VerifiablePresentation) -> Result<bool> {
        self.credential_manager.verify_presentation(presentation).await
    }
}

/// Create a simple example DID
///
/// This is a convenience function for creating a DID quickly.
/// For more control, use the DidManager directly.
pub async fn create_did(wallet: Arc<BitcoinWallet>) -> Result<DidDocument> {
    let did_manager = DidManager::new(wallet, Network::Mainnet);
    did_manager.create_did().await
}

/// Issue a simple credential
///
/// This is a convenience function for issuing a credential quickly.
/// For more control, use the CredentialManager directly.
pub async fn issue_credential(
    issuer_did: &str,
    subject_did: &str,
    credential_type: &str,
    claims: serde_json::Value,
) -> Result<VerifiableCredential> {
    let wallet = crate::wallet::BitcoinWallet::default_wallet().await?;
    let wallet_arc = Arc::new(wallet);
    
    let did_manager = Arc::new(DidManager::new(wallet_arc, Network::Mainnet));
    let credential_manager = CredentialManager::new(did_manager);
    
    let mut claims_map = std::collections::HashMap::new();
    if let serde_json::Value::Object(obj) = claims {
        for (key, value) in obj {
            claims_map.insert(key, value);
        }
    }
    
    credential_manager.issue_credential(
        issuer_did,
        subject_did,
        credential_type,
        claims_map,
        Some(365), // Valid for 1 year
    ).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::wallet::WalletConfig;
    
    #[tokio::test]
    async fn test_web5_manager() {
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
        
        // Create Web5 manager
        let web5_manager = Web5Manager::new(wallet_arc, Network::Testnet);
        
        // Create DIDs
        let issuer_did_doc = web5_manager.create_did().await.unwrap();
        let subject_did_doc = web5_manager.create_did().await.unwrap();
        
        // Create claims
        let mut claims = std::collections::HashMap::new();
        claims.insert("name".to_string(), serde_json::json!("Alice"));
        claims.insert("email".to_string(), serde_json::json!("alice@example.com"));
        
        // Issue a credential
        let credential = web5_manager.issue_credential(
            &issuer_did_doc.id,
            &subject_did_doc.id,
            "EmailCredential",
            claims,
            Some(365),
        ).await.unwrap();
        
        // Verify the credential
        let is_valid = web5_manager.verify_credential(&credential).await.unwrap();
        assert!(is_valid);
        
        // Create a presentation
        let presentation = web5_manager.create_presentation(
            &subject_did_doc.id,
            vec![credential],
        ).await.unwrap();
        
        // Verify the presentation
        let is_valid = web5_manager.verify_presentation(&presentation).await.unwrap();
        assert!(is_valid);
    }
}
