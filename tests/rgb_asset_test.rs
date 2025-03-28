#![feature(edition2021)]
// RGB Asset Transfer Tests

#[cfg(feature = "rgb")]
mod rgb_tests {
    use anya_core::dependencies::anya_bitcoin::rgb::RgbManager;
    use anya_core::dependencies::anya_bitcoin::wallet::{BitcoinWallet, WalletConfig};
    use bitcoin::Network;
    use tempfile::tempdir;
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use chrono::Utc;

    #[tokio::test]
    async fn test_rgb_asset_issuance() -> anyhow::Result<()> {
        // Create temp directory for the test
        let temp_dir = tempdir()?;
        
        // Create a wallet for testing
        let wallet_config = WalletConfig {
            name: "rgb-test-wallet".to_string(),
            database_path: temp_dir.path().join("rgb-wallet.db"),
            network: Network::Regtest,
            electrum_url: Some("ssl://electrum.blockstream.info:60002".to_string()),
            descriptor: Some("wpkh([73c5da0a/84'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/0/*)".to_string()),
            change_descriptor: None,
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        
        // Sync the wallet to ensure we have the latest state
        wallet.sync().await?;
        
        // Create RGB manager
        let rgb_manager = RgbManager::new(Network::Regtest, temp_dir.path())?;
        
        // Issue a new RGB asset
        println!("Issuing RGB asset...");
        let asset_info = rgb_manager.issue_asset(
            &wallet,
            "Test Token",
            Some("TST"),
            1_000_000, // 1 million units
            Some("Test token for RGB testing")
        ).await?;
        
        // Verify asset was created
        assert_eq!(asset_info.name, "Test Token");
        assert_eq!(asset_info.ticker.unwrap_or_default(), "TST");
        assert_eq!(asset_info.total_supply, 1_000_000);
        
        println!("RGB asset issued successfully: {}", asset_info.contract_id);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_rgb_asset_transfer_with_metadata() -> anyhow::Result<()> {
        // Create temp directory for the test
        let temp_dir = tempdir()?;
        
        // Create a wallet for testing
        let wallet_config = WalletConfig {
            name: "rgb-transfer-test-wallet".to_string(),
            database_path: temp_dir.path().join("rgb-transfer-wallet.db"),
            network: Network::Regtest,
            electrum_url: Some("ssl://electrum.blockstream.info:60002".to_string()),
            descriptor: Some("wpkh([73c5da0a/84'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/0/*)".to_string()),
            change_descriptor: None,
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        
        // Sync the wallet to ensure we have the latest state
        wallet.sync().await?;
        
        // Create RGB manager
        let rgb_manager = RgbManager::new(Network::Regtest, temp_dir.path())?;
        
        // Issue a new RGB asset
        println!("Issuing RGB asset for transfer test...");
        let asset_info = rgb_manager.issue_asset(
            &wallet,
            "Transfer Test Token",
            Some("TTT"),
            1_000_000, // 1 million units
            Some("Test token for RGB transfer testing")
        ).await?;
        
        // Get a new address for the transfer recipient
        let recipient_address = wallet.get_new_address().await?.address.to_string();
        
        // Create metadata for the transfer
        let mut metadata = HashMap::new();
        metadata.insert("transfer_purpose".to_string(), "test transfer".to_string());
        metadata.insert("transfer_id".to_string(), "12345".to_string());
        metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
        
        // Transfer the asset with metadata
        println!("Transferring RGB asset with metadata...");
        let txid = rgb_manager.transfer_asset_with_metadata(
            &wallet,
            &asset_info.contract_id,
            &recipient_address,
            50_000, // Transfer 50,000 units
            &metadata
        ).await?;
        
        println!("RGB asset transferred successfully: {}", txid);
        
        // Verify the transfer by getting updated asset info
        let updated_asset_info = rgb_manager.get_asset_info(&asset_info.contract_id).await?;
        
        // The balance should be updated after the transfer
        assert_eq!(updated_asset_info.balance, 950_000); // Original 1,000,000 - 50,000 transferred
        
        Ok(())
    }
}
