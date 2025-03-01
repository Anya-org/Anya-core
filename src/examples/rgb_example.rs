// RGB Asset Transfer Examples

#[cfg(feature = "rgb")]
pub mod rgb_examples {
    use crate::bitcoin::wallet::BitcoinWallet;
    use crate::bitcoin::wallet::WalletConfig;
    use crate::bitcoin::rgb::RgbManager;
    use crate::bitcoin::rgb::AssetInfo;
    use crate::bitcoin::rgb::AssetMetadata;
    use bitcoin::Network;
    use std::collections::HashMap;
    use std::path::Path;
    use std::sync::Arc;

    // Example: Issue a new RGB asset
    #[allow(dead_code)]
    pub async fn issue_rgb_asset(wallet: Arc<BitcoinWallet>) -> anyhow::Result<AssetInfo> {
        // Create RGB manager
        let data_dir = std::env::temp_dir().join("rgb_example");
        std::fs::create_dir_all(&data_dir)?;
        
        let rgb_manager = RgbManager::new(Network::Testnet, &data_dir)?;
        
        // Issue a new RGB asset
        println!("Issuing RGB asset...");
        let asset_info = rgb_manager.issue_asset(
            &wallet,
            "Example Token",
            Some("EXT"),
            1_000_000, // 1 million units
            Some("Example token for demonstration purposes")
        ).await?;
        
        println!("RGB asset issued successfully!");
        println!("Asset ID: {}", asset_info.contract_id);
        println!("Name: {}", asset_info.name);
        println!("Ticker: {}", asset_info.ticker.as_deref().unwrap_or("N/A"));
        println!("Total supply: {}", asset_info.total_supply);
        
        Ok(asset_info)
    }

    // Example: Transfer RGB asset with metadata
    #[allow(dead_code)]
    pub async fn transfer_rgb_asset_with_metadata(
        wallet: Arc<BitcoinWallet>,
        contract_id: &str,
        recipient_address: &str,
        amount: u64
    ) -> anyhow::Result<String> {
        // Create RGB manager
        let data_dir = std::env::temp_dir().join("rgb_example");
        let rgb_manager = RgbManager::new(Network::Testnet, &data_dir)?;
        
        // Create metadata for the transfer
        let mut metadata = HashMap::new();
        metadata.insert("transfer_purpose".to_string(), "example transfer".to_string());
        metadata.insert("transfer_id".to_string(), uuid::Uuid::new_v4().to_string());
        metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        
        // Transfer the asset with metadata
        println!("Transferring RGB asset with metadata...");
        let txid = rgb_manager.transfer_asset_with_metadata(
            &wallet,
            contract_id,
            recipient_address,
            amount,
            &metadata
        ).await?;
        
        println!("RGB asset transferred successfully!");
        println!("Transaction ID: {}", txid);
        
        Ok(txid)
    }

    // Example: Get RGB asset information
    #[allow(dead_code)]
    pub async fn get_asset_info(contract_id: &str) -> anyhow::Result<AssetInfo> {
        // Create RGB manager
        let data_dir = std::env::temp_dir().join("rgb_example");
        let rgb_manager = RgbManager::new(Network::Testnet, &data_dir)?;
        
        // Get asset information
        let asset_info = rgb_manager.get_asset_info(contract_id).await?;
        
        println!("RGB asset information:");
        println!("Asset ID: {}", asset_info.contract_id);
        println!("Name: {}", asset_info.name);
        println!("Ticker: {}", asset_info.ticker.as_deref().unwrap_or("N/A"));
        println!("Total supply: {}", asset_info.total_supply);
        println!("Current balance: {}", asset_info.balance);
        
        Ok(asset_info)
    }

    // Example: Get transfer metadata
    #[allow(dead_code)]
    pub async fn get_transfer_metadata(
        contract_id: &str,
        txid: &str
    ) -> anyhow::Result<Option<AssetMetadata>> {
        // Create RGB manager
        let data_dir = std::env::temp_dir().join("rgb_example");
        let rgb_manager = RgbManager::new(Network::Testnet, &data_dir)?;
        
        // Get transfer metadata
        let metadata = rgb_manager.get_transfer_metadata(contract_id, txid).await?;
        
        if let Some(metadata) = &metadata {
            println!("Transfer metadata found:");
            for (key, value) in &metadata.fields {
                println!("  {}: {}", key, value);
            }
        } else {
            println!("No metadata found for this transfer");
        }
        
        Ok(metadata)
    }

    // Helper to create a test wallet
    #[allow(dead_code)]
    pub fn create_test_wallet() -> anyhow::Result<Arc<BitcoinWallet>> {
        let data_dir = std::env::temp_dir().join("wallet_example");
        std::fs::create_dir_all(&data_dir)?;
        
        let wallet_config = WalletConfig {
            name: "test-wallet".to_string(),
            database_path: data_dir.join("wallet.db"),
            network: Network::Testnet,
            electrum_url: Some("ssl://electrum.blockstream.info:60002".to_string()),
            descriptor: Some("wpkh([73c5da0a/84'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/0/*)".to_string()),
            change_descriptor: None,
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        Ok(wallet)
    }
}
