// RGB Asset Transfer Tests

#[cfg(any(feature = "bitcoin", feature = "complete"))]
mod rgb_tests {
    use anya_core::layer2::rgb::ContractManager;
    use tempfile::tempdir;

    // Test constants
    const SENDER_ADDRESS: &str = "tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
    const RECIPIENT_ADDRESS: &str = "tb1q6kj3wqj5r7c0h5lt6l5q6vzv8cj3egl2kpmnd7";

    #[tokio::test]
    async fn test_rgb_asset_issuance() -> anyhow::Result<()> {
        // Create temp directory for the test
        let _temp_dir = tempdir()?;

        // Create RGB contract manager
        let contract_manager = ContractManager::new();

        // Create a new RGB asset
        println!("Creating RGB asset...");
        let asset = contract_manager.create_asset(
            "tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", // Example testnet address
            1000000,                                      // total supply
            8,                                            // precision
            "Test Token",                                 // metadata/description
        )?;

        // Verify asset was created
        assert_eq!(asset.name, "Test Token");
        assert_eq!(asset.precision, 8);
        assert_eq!(asset.issued_supply, 0); // Not issued yet, just created

        println!("RGB asset created successfully: {}", asset.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_rgb_asset_transfer_with_metadata() -> anyhow::Result<()> {
        // Create temp directory for the test
        let _temp_dir = tempdir()?;

        // Create RGB contract manager
        let contract_manager = ContractManager::new();

        // Create a new RGB asset for transfer testing
        println!("Creating RGB asset for transfer test...");
        let _asset = contract_manager.create_asset(
            "tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", // Example testnet address
            1000000,                                      // total supply
            8,                                            // precision
            "Transfer Test Token",                        // metadata/description
        )?;

        // Get a new address for the transfer recipient
        // Get a new address for the transfer recipient
        let sender_address = SENDER_ADDRESS.to_string();
        let recipient_address = RECIPIENT_ADDRESS.to_string();

        // Create a transfer using the contract manager
        println!("Creating RGB asset transfer...");
        let transfer = contract_manager.transfer_asset(
            &sender_address,
            &recipient_address,
            50_000, // Transfer 50,000 units
        )?;

        println!(
            "RGB asset transfer created successfully with nonce: {}",
            transfer.nonce
        );

        // Verify the transfer object
        assert_eq!(transfer.from, sender_address);
        assert_eq!(transfer.to, recipient_address);
        assert_eq!(transfer.amount, 50_000);

        Ok(())
    }
}
