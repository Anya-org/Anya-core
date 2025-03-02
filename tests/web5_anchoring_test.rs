// Web5 Bitcoin anchoring test
#[cfg(feature = "web5")]
mod web5_tests {
    use anya_core::dependencies::anya_bitcoin::web5::enhanced_dwn::{EnhancedDwn, EnhancedDwnOptions};
    use anya_core::dependencies::anya_bitcoin::web5::credential::CredentialManager;
    use anya_core::dependencies::anya_bitcoin::web5::did::DidManager;
    use anya_core::dependencies::anya_bitcoin::wallet::BitcoinWallet;
    use anya_core::dependencies::anya_bitcoin::wallet::WalletConfig;
    use bitcoin::Network;
    use bdk::wallet::AddressIndex;
    use tempfile::tempdir;
    use std::sync::Arc;
    use std::collections::HashMap;
    use serde_json::Value;

    #[tokio::test]
    async fn test_web5_credential_with_bitcoin_anchoring() -> anyhow::Result<()> {
        // Create temp directory for the test
        let temp_dir = tempdir()?;
        
        // Initialize DID Manager
        let did_manager = Arc::new(DidManager::new(temp_dir.path().join("dids"))?);
        
        // Create a wallet for testing - using BDK's taproot functionality
        let wallet_config = WalletConfig {
            name: "test-wallet".to_string(),
            database_path: temp_dir.path().join("wallet.db"),
            network: Network::Regtest,
            electrum_url: Some("ssl://electrum.blockstream.info:60002".to_string()),
            descriptor: Some("tr([73c5da0a/86'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/0/*)".to_string()),
            change_descriptor: None,
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        
        // Create credential manager with Bitcoin anchoring
        let credential_manager = CredentialManager::with_bitcoin_anchoring(
            did_manager.clone(),
            wallet.clone(),
            Network::Regtest
        );
        
        // Create issuer and subject DIDs
        let issuer_did = did_manager.create_did("key").await?;
        let subject_did = did_manager.create_did("key").await?;
        
        // Create claims for credential
        let mut claims = HashMap::new();
        claims.insert("name".to_string(), Value::String("Alice".to_string()));
        claims.insert("age".to_string(), Value::Number(25.into()));
        
        // Issue a credential with Bitcoin anchoring
        println!("Issuing credential with Bitcoin anchoring...");
        let credential = credential_manager.issue_anchored_credential(
            &issuer_did,
            &subject_did,
            "TestCredential",
            claims,
            Some(365) // Valid for 1 year
        ).await?;
        
        // Verify the credential has Bitcoin anchoring info
        assert!(credential.bitcoin_anchoring.is_some());
        println!("Credential issued successfully with Bitcoin anchoring");
        
        // Verify the credential
        let is_valid = credential_manager.verify_credential(&credential).await?;
        assert!(is_valid);
        println!("Credential verified successfully");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_output_psbt_creation() -> anyhow::Result<()> {
        // Create temp directory for the test
        let temp_dir = tempdir()?;
        
        // Create a wallet for testing with BDK's taproot support
        // Using BDK 0.30.0 API
        let wallet_config = WalletConfig {
            name: "multi-output-test-wallet".to_string(),
            database_path: temp_dir.path().join("multi-output-wallet.db"),
            network: Network::Regtest,
            electrum_url: Some("ssl://electrum.blockstream.info:60002".to_string()),
            descriptor: Some("tr([73c5da0a/86'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/0/*)".to_string()),
            change_descriptor: None,
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        
        // Sync the wallet to ensure we have the latest state
        wallet.sync().await?;
        
        // Generate some testing addresses
        let addr1 = wallet.get_address(AddressIndex::New).await?.address;
        let addr2 = wallet.get_address(AddressIndex::New).await?.address;
        let addr3 = wallet.get_address(AddressIndex::New).await?.address;
        
        // Define multiple outputs
        let outputs = vec![
            (addr1.to_string(), 10000),  // 10,000 sats
            (addr2.to_string(), 20000),  // 20,000 sats
            (addr3.to_string(), 15000),  // 15,000 sats
        ];
        
        // Create multi-output PSBT
        println!("Creating multi-output PSBT...");
        let psbt = wallet.create_multi_output_psbt(outputs, Some(1.0)).await?;
        
        // Check that the PSBT has the correct number of outputs
        assert_eq!(psbt.unsigned_tx().output.len(), 4); // 3 destinations + change output
        
        // Enhance the PSBT for hardware wallet compatibility
        println!("Enhancing PSBT for hardware wallet compatibility...");
        let mut enhanced_psbt = psbt.clone();
        wallet.enhance_psbt_for_hardware(&mut enhanced_psbt).await?;
        
        // Check if the PSBT was properly enhanced with BDK 0.30.0
        assert!(enhanced_psbt.inputs().len() > 0);
        
        println!("Multi-output PSBT created and enhanced successfully");
        
        Ok(())
    }
}
