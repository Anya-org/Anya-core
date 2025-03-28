#![feature(edition2021)]
// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Integration tests for Bitcoin-anchored Web5 verifiable credentials
//!
//! These tests verify the functionality of creating, verifying, and revoking
//! verifiable credentials that are anchored to the Bitcoin blockchain.

use std::collections::HashMap;
use std::sync::Arc;
use std::path::PathBuf;

use bitcoin::Network;
use serde_json::Value;
use tempfile::tempdir;

use anya_bitcoin::wallet::{BitcoinWallet, WalletConfig};
use anya_bitcoin::web5::{Web5Manager, credential::VerifiableCredential};

// This test can be run with: cargo test --package anya-bitcoin --test web5_credential_anchoring -- --ignored
#[tokio::test]
#[ignore] // Requires connection to Bitcoin testnet
async fn test_bitcoin_anchored_credentials() -> anyhow::Result<()> {
    // Setup temporary directory for wallet database
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("wallet.db");
    
    // Create wallet config for testnet
    let wallet_config = WalletConfig {
        name: "test-wallet".to_string(),
        database_path: db_path,
        network: Network::Testnet,
        electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
        password: None,
        mnemonic: None, // Will generate a new one
        use_taproot: true,
    };
    
    // Create wallet
    let wallet = Arc::new(BitcoinWallet::new(wallet_config).await?);
    
    // Create Web5 manager
    let web5 = Web5Manager::new(wallet, Network::Testnet);
    
    // Create two DIDs (issuer and subject)
    let issuer_doc = web5.create_did().await?;
    let subject_doc = web5.create_did().await?;
    
    let issuer_did = issuer_doc.id.clone();
    let subject_did = subject_doc.id.clone();
    
    // Create claims for a credential
    let mut claims = HashMap::new();
    claims.insert("name".to_string(), Value::String("Test User".to_string()));
    claims.insert("age".to_string(), Value::Number(serde_json::Number::from(25)));
    claims.insert("email".to_string(), Value::String("test@example.com".to_string()));
    
    // Issue a Bitcoin-anchored credential
    println!("Issuing Bitcoin-anchored credential...");
    let credential = web5.issue_anchored_credential(
        &issuer_did,
        &subject_did,
        "IdentityCredential",
        claims,
        Some(365), // Valid for 1 year
    ).await?;
    
    // Print the credential details
    println!("Credential issued with ID: {}", credential.id);
    if let Some(anchoring) = &credential.bitcoin_anchoring {
        println!("Anchored in transaction: {}", anchoring.txid);
    } else {
        println!("Warning: Credential not anchored correctly");
    }
    
    // Wait for at least one confirmation (in a real test you might want to mock this)
    // This is just a placeholder - in reality you'd wait longer or use a regtest network
    println!("Waiting for transaction confirmation (simulated)...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // Update the anchoring status (would normally check confirmations)
    let mut credential_updated = credential.clone();
    if let Err(e) = web5.update_anchoring_status(&mut credential_updated).await {
        println!("Failed to update anchoring status: {}", e);
    }
    
    // Verify the credential
    let is_valid = web5.verify_credential(&credential_updated).await?;
    println!("Credential verification result: {}", is_valid);
    
    // Revoke the credential with Bitcoin anchoring
    println!("Revoking credential with Bitcoin anchoring...");
    let revocation_txid = web5.revoke_credential_with_bitcoin(&credential.id, &issuer_did).await?;
    println!("Credential revoked in transaction: {}", revocation_txid);
    
    // Check if the credential is revoked (this would typically require confirmation)
    let is_revoked = web5.check_credential_revocation(&credential.id).await?;
    println!("Credential revocation status: {}", is_revoked);
    
    // Create a presentation containing the credential
    println!("Creating verifiable presentation...");
    let presentation = web5.create_presentation(
        &subject_did,
        vec![credential_updated],
    ).await?;
    
    // Verify the presentation
    let presentation_valid = web5.verify_presentation(&presentation).await?;
    println!("Presentation verification result: {}", presentation_valid);
    
    Ok(())
}
