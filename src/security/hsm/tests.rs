#![feature(edition2021)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::security::hsm::{
        HsmManager, 
        config::HsmConfig,
        provider::{KeyGenParams, KeyType, KeyUsage, EcCurve, SigningAlgorithm},
        bitcoin::{BitcoinHsmProvider, BitcoinHsmConfig, BitcoinKeyType, BitcoinNetwork, BitcoinSignatureType},
    };
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_hsm_initialization() {
        // Create HSM configuration
        let config = HsmConfig::development();
        
        // Create HSM manager
        let hsm_manager = HsmManager::new(config);
        
        // Initialize HSM manager
        let result = hsm_manager.initialize().await;
        assert!(result.is_ok(), "HSM initialization failed: {:?}", result);
        
        // Check HSM status
        let status = hsm_manager.get_status().await;
        assert_eq!(status, HsmStatus::Ready, "HSM status should be Ready");
        
        // Close HSM manager
        let result = hsm_manager.close().await;
        assert!(result.is_ok(), "HSM close failed: {:?}", result);
        
        // Check HSM status
        let status = hsm_manager.get_status().await;
        assert_eq!(status, HsmStatus::ShutDown, "HSM status should be ShutDown");
    }
    
    #[tokio::test]
    async fn test_key_operations() {
        // Create HSM configuration
        let config = HsmConfig::development();
        
        // Create HSM manager
        let hsm_manager = HsmManager::new(config);
        
        // Initialize HSM manager
        let result = hsm_manager.initialize().await;
        assert!(result.is_ok(), "HSM initialization failed: {:?}", result);
        
        // Generate key pair
        let key_params = KeyGenParams {
            id: Some("test-key-1".to_string()),
            label: "Test Key 1".to_string(),
            key_type: KeyType::Ec { curve: EcCurve::P256 },
            extractable: false,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: HashMap::new(),
        };
        
        let result = hsm_manager.generate_key_pair(key_params).await;
        assert!(result.is_ok(), "Key generation failed: {:?}", result);
        
        let public_key_info = result.unwrap();
        assert_eq!(public_key_info.id, "test-key-1", "Key ID should match");
        
        // Sign data
        let data = b"Test data to sign";
        let result = hsm_manager.sign("test-key-1", SigningAlgorithm::EcdsaSha256, data).await;
        assert!(result.is_ok(), "Signing failed: {:?}", result);
        
        let signature = result.unwrap();
        
        // Verify signature
        let result = hsm_manager.verify("test-key-1", SigningAlgorithm::EcdsaSha256, data, &signature).await;
        assert!(result.is_ok(), "Verification failed: {:?}", result);
        assert!(result.unwrap(), "Signature should be valid");
        
        // Close HSM manager
        let result = hsm_manager.close().await;
        assert!(result.is_ok(), "HSM close failed: {:?}", result);
    }
    
    // This test is commented out as it requires the BitcoinHsmProvider to be fully implemented
    // with a working base provider. Uncomment and adapt when implementing the full functionality.
    /*
    #[tokio::test]
    async fn test_bitcoin_operations() {
        // Create HSM configuration
        let config = HsmConfig::development();
        
        // Create HSM manager
        let hsm_manager = HsmManager::new(config);
        
        // Initialize HSM manager
        let result = hsm_manager.initialize().await;
        assert!(result.is_ok(), "HSM initialization failed: {:?}", result);
        
        // Create Bitcoin HSM provider
        let base_provider = Arc::new(hsm_manager);
        let bitcoin_config = BitcoinHsmConfig {
            base_provider,
            network: BitcoinNetwork::Testnet,
            derivation_path_template: "m/86'/0'/0'/0/{}".to_string(),
            use_taproot: true,
            miniscript_policy_template: Some("and(pk(@0),or(pk(@1),after(144)))".to_string()),
            default_key_type: BitcoinKeyType::Taproot,
        };
        
        let bitcoin_provider = BitcoinHsmProvider::new(bitcoin_config);
        
        // Generate Bitcoin key
        let result = bitcoin_provider.generate_bitcoin_key(
            "test",
            Some(BitcoinKeyType::Taproot),
            Some(0)
        ).await;
        assert!(result.is_ok(), "Bitcoin key generation failed: {:?}", result);
        
        let bitcoin_key = result.unwrap();
        assert_eq!(bitcoin_key.key_type, BitcoinKeyType::Taproot, "Key type should be Taproot");
        
        // Sign Bitcoin message
        let message = "Test Bitcoin message";
        let result = bitcoin_provider.sign_bitcoin_message(
            &bitcoin_key.key_id,
            message,
            true
        ).await;
        assert!(result.is_ok(), "Bitcoin message signing failed: {:?}", result);
        
        // Sign Bitcoin transaction
        let tx_hex = "020000000001..."; // Placeholder transaction hex
        let result = bitcoin_provider.sign_bitcoin_transaction(
            &bitcoin_key.key_id,
            tx_hex,
            BitcoinSignatureType::Schnorr,
            0x01 // SIGHASH_ALL
        ).await;
        assert!(result.is_ok(), "Bitcoin transaction signing failed: {:?}", result);
        
        // Create Taproot asset
        let result = crate::security::create_taproot_asset(
            &bitcoin_provider,
            "Test Asset",
            1000000
        ).await;
        assert!(result.is_ok(), "Taproot asset creation failed: {:?}", result);
        
        let asset_id = result.unwrap();
        assert!(!asset_id.is_empty(), "Asset ID should not be empty");
    }
    */
    
    #[tokio::test]
    async fn test_audit_logging() {
        // Create HSM configuration
        let config = HsmConfig::development();
        
        // Create HSM manager
        let hsm_manager = HsmManager::new(config);
        
        // Initialize HSM manager
        let result = hsm_manager.initialize().await;
        assert!(result.is_ok(), "HSM initialization failed: {:?}", result);
        
        // Generate key pair to create some audit logs
        let key_params = KeyGenParams {
            id: Some("test-key-2".to_string()),
            label: "Test Key 2".to_string(),
            key_type: KeyType::Ec { curve: EcCurve::P256 },
            extractable: false,
            usages: vec![KeyUsage::Sign, KeyUsage::Verify],
            expires_at: None,
            attributes: HashMap::new(),
        };
        
        let result = hsm_manager.generate_key_pair(key_params).await;
        assert!(result.is_ok(), "Key generation failed: {:?}", result);
        
        // Get audit events
        let filter = crate::security::hsm::audit::AuditFilter::default();
        let result = hsm_manager.get_audit_events(filter).await;
        assert!(result.is_ok(), "Getting audit events failed: {:?}", result);
        
        let events = result.unwrap();
        assert!(!events.is_empty(), "There should be audit events");
        
        // Check if initialization event is present
        let init_events: Vec<_> = events.iter()
            .filter(|e| e.event_type == "Initialize")
            .collect();
        assert!(!init_events.is_empty(), "There should be an initialization event");
        
        // Close HSM manager
        let result = hsm_manager.close().await;
        assert!(result.is_ok(), "HSM close failed: {:?}", result);
    }
    
    #[test]
    fn test_parse_key_type() {
        // Test parsing various key types
        let result = KeyType::from_str("rsa/2048");
        assert!(result.is_ok(), "Parsing RSA key type failed: {:?}", result);
        assert!(matches!(result.unwrap(), KeyType::Rsa { bits: 2048 }));
        
        let result = KeyType::from_str("ec/p256");
        assert!(result.is_ok(), "Parsing EC key type failed: {:?}", result);
        assert!(matches!(result.unwrap(), KeyType::Ec { curve: EcCurve::P256 }));
        
        let result = KeyType::from_str("aes/256");
        assert!(result.is_ok(), "Parsing AES key type failed: {:?}", result);
        assert!(matches!(result.unwrap(), KeyType::Aes { bits: 256 }));
        
        let result = KeyType::from_str("invalid");
        assert!(result.is_err(), "Parsing invalid key type should fail");
    }
}

/// Example usage of the HSM Manager
/// 
/// This function demonstrates how to use the HSM Manager for common operations.
/// It follows the security requirements specified in the Bitcoin Development Framework v2.5.
/// 
/// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
pub async fn hsm_example_usage() -> Result<(), crate::security::hsm::error::HsmError> {
    use crate::security::hsm::{
        HsmManager, 
        config::HsmConfig,
        provider::{KeyGenParams, KeyType, KeyUsage, EcCurve, SigningAlgorithm},
    };
    use std::collections::HashMap;
    
    // Step 1: Create HSM configuration
    let config = HsmConfig::development();
    
    // Step 2: Create HSM manager
    let hsm_manager = HsmManager::new(config);
    
    // Step 3: Initialize HSM manager
    hsm_manager.initialize().await?;
    
    // Step 4: Generate key pair for signing
    let mut attributes = HashMap::new();
    attributes.insert("purpose".to_string(), "signature".to_string());
    attributes.insert("application".to_string(), "anya-core".to_string());
    
    let key_params = KeyGenParams {
        id: Some("signing-key-1".to_string()),
        label: "Anya Core Signing Key".to_string(),
        key_type: KeyType::Ec { curve: EcCurve::P256 },
        extractable: false,
        usages: vec![KeyUsage::Sign, KeyUsage::Verify],
        expires_at: None,
        attributes,
    };
    
    let public_key_info = hsm_manager.generate_key_pair(key_params).await?;
    println!("Generated key: {}", public_key_info.id);
    
    // Step 5: Sign data
    let data_to_sign = b"Important data that needs to be signed";
    let signature = hsm_manager.sign(
        &public_key_info.id,
        SigningAlgorithm::EcdsaSha256,
        data_to_sign
    ).await?;
    println!("Signature length: {} bytes", signature.len());
    
    // Step 6: Verify signature
    let is_valid = hsm_manager.verify(
        &public_key_info.id,
        SigningAlgorithm::EcdsaSha256,
        data_to_sign,
        &signature
    ).await?;
    println!("Signature valid: {}", is_valid);
    
    // Step 7: List all keys
    let keys = hsm_manager.list_keys().await?;
    println!("Number of keys: {}", keys.len());
    
    // Step 8: Get audit events
    use crate::security::hsm::audit::AuditFilter;
    
    let filter = AuditFilter {
        limit: Some(10),
        ..Default::default()
    };
    
    let events = hsm_manager.get_audit_events(filter).await?;
    println!("Number of audit events: {}", events.len());
    
    // Step 9: Rotate key
    let new_public_key_info = hsm_manager.rotate_key(&public_key_info.id).await?;
    println!("Rotated key: {}", new_public_key_info.id);
    
    // Step 10: Close HSM manager
    hsm_manager.close().await?;
    
    Ok(())
}

/// Example of Bitcoin operations with HSM
/// 
/// This function demonstrates how to use the HSM for Bitcoin-specific operations.
/// It follows the security requirements specified in the Bitcoin Development Framework v2.5.
/// 
/// [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]
pub async fn bitcoin_hsm_example() -> Result<(), crate::security::hsm::error::HsmError> {
    use std::sync::Arc;
    use crate::security::hsm::{
        HsmManager, 
        config::HsmConfig,
        bitcoin::{
            BitcoinHsmProvider, BitcoinHsmConfig, BitcoinKeyType, 
            BitcoinNetwork, BitcoinSignatureType, TaprootScriptTree,
            TaprootScriptNode, BitcoinSpvProof, DlcParams, DlcContractInfo,
            DlcOutcome, DlcCetInfo
        }
    };
    
    // Step 1: Create and initialize HSM manager
    let config = HsmConfig::development();
    let hsm_manager = HsmManager::new(config);
    hsm_manager.initialize().await?;
    
    // Step 2: Create Bitcoin HSM provider
    let base_provider = Arc::new(hsm_manager);
    let bitcoin_config = BitcoinHsmConfig {
        base_provider,
        network: BitcoinNetwork::Testnet,
        derivation_path_template: "m/86'/0'/0'/0/{}".to_string(),
        use_taproot: true,
        miniscript_policy_template: Some("and(pk(@0),or(pk(@1),after(144)))".to_string()),
        default_key_type: BitcoinKeyType::Taproot,
    };
    
    let bitcoin_provider = BitcoinHsmProvider::new(bitcoin_config);
    
    // Step 3: Generate Bitcoin key
    let bitcoin_key = bitcoin_provider.generate_bitcoin_key(
        "wallet",
        Some(BitcoinKeyType::Taproot),
        Some(0)
    ).await?;
    
    println!("Generated Bitcoin key: {}", bitcoin_key.key_id);
    println!("Bitcoin address: {}", bitcoin_key.script_details.address);
    
    // Step 4: Sign a Bitcoin message
    let message = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
    let signature = bitcoin_provider.sign_bitcoin_message(
        &bitcoin_key.key_id,
        message,
        true
    ).await?;
    
    println!("Message signature: {}", signature);
    
    // Step 5: Create a Taproot script tree
    let script_tree = TaprootScriptTree {
        root: TaprootScriptNode::Branch {
            left: Box::new(TaprootScriptNode::Leaf {
                script: "OP_SHA256 d7314c8d09f8c6c7f8275e3887a3b9e8a472d71ce2c0e33e1e2fb2f626e5cda2 OP_EQUALVERIFY OP_CHECKSIG".to_string(),
                version: 0xc0,
            }),
            right: Box::new(TaprootScriptNode::Leaf {
                script: "OP_CHECKSIG OP_IFDUP OP_NOTIF OP_DUP OP_HASH160 6c88c14b290f56e789f2882bc835e757df533abc OP_EQUALVERIFY OP_CHECKSIGVERIFY OP_ENDIF".to_string(),
                version: 0xc0,
            }),
        },
    };
    
    // Step 6: Create a Taproot output
    let taproot_output = bitcoin_provider.create_taproot_output(
        &bitcoin_key.key_id,
        Some(script_tree)
    ).await?;
    
    println!("Taproot output address: {}", taproot_output.address);
    
    // Step 7: Create a Taproot asset
    let asset_id = crate::security::create_taproot_asset(
        &bitcoin_provider,
        r#"{"name":"Anya Token","ticker":"ANY","description":"Anya Core Governance Token"}"#,
        21000000 // Total supply
    ).await?;
    
    println!("Created Taproot asset: {}", asset_id);
    
    // Step 8: Create a DLC (Discreet Log Contract)
    use chrono::{Utc, Duration};
    
    let maturity_time = Utc::now() + Duration::days(30);
    
    let dlc_params = DlcParams {
        oracle_public_keys: vec!["03a7d52dbac0dbc90578269f4b8a307ef298bbe3f7a7e3fa5db7631fd7f8ea6b5f".to_string()],
        oracle_r_points: vec!["031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f".to_string()],
        contract_info: DlcContractInfo {
            descriptor: "Bitcoin price at maturity".to_string(),
            outcomes: vec![
                DlcOutcome {
                    value: "BTC < $30,000".to_string(),
                    payout_a: 900000, // 0.9 BTC
                    payout_b: 100000, // 0.1 BTC
                },
                DlcOutcome {
                    value: "$30,000 <= BTC < $40,000".to_string(),
                    payout_a: 500000, // 0.5 BTC
                    payout_b: 500000, // 0.5 BTC
                },
                DlcOutcome {
                    value: "BTC >= $40,000".to_string(),
                    payout_a: 100000, // 0.1 BTC
                    payout_b: 900000, // 0.9 BTC
                },
            ],
            maturity_time,
        },
        cets: vec![
            DlcCetInfo {
                outcome_index: 0,
                tx_hex: "0200000001...".to_string(), // Placeholder
                adaptor_sig_a: None,
                adaptor_sig_b: None,
            },
            DlcCetInfo {
                outcome_index: 1,
                tx_hex: "0200000001...".to_string(), // Placeholder
                adaptor_sig_a: None,
                adaptor_sig_b: None,
            },
            DlcCetInfo {
                outcome_index: 2,
                tx_hex: "0200000001...".to_string(), // Placeholder
                adaptor_sig_a: None,
                adaptor_sig_b: None,
            },
        ],
    };
    
    let dlc_info = crate::security::hsm::bitcoin::create_dlc(
        &bitcoin_provider,
        &bitcoin_key.key_id,
        dlc_params
    ).await?;
    
    println!("Created DLC: {}", dlc_info.dlc_id);
    
    // Step 9: Verify a Bitcoin payment SPV proof
    let spv_proof = BitcoinSpvProof {
        tx_hash: "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d".to_string(),
        block_header: "0000002006226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f7231332140a4d3d3d11d54e559b5fb3bcf5e1c1c0d2e8331f49ad0e733841a975fd2f62ffff7f2001000000".to_string(),
        merkle_proof: vec![
            "8d5dbf5f9d0ef82f30cfc5166e34b915212e08b6559f19cad316d455db75a0a1".to_string(),
            "28e8dab64c5a2a0e0f1e9c3b6bcf7a6c3c7d2c0f2e411316e2e57d5a9dce02a3".to_string(),
        ],
        block_height: 680000,
        confirmations: 10,
    };
    
    let is_valid = bitcoin_provider.verify_bitcoin_spv_proof(spv_proof).await?;
    println!("SPV proof valid: {}", is_valid);
    
    Ok(())
} 