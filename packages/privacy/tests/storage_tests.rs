#[cfg(test)]
mod tests {
    use anya_privacy::silent_payments::{
        KeyManager, SilentPaymentScanner, SilentPaymentInfo
    };
    use bitcoin::{Network, OutPoint, TxOut, Txid};
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;
    use tempfile::tempdir;
    
    #[test]
    fn test_save_and_load_scanner() -> anya_privacy::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_storage.json");
        
        // Create a scanner
        let key_manager = KeyManager::new_random()?;
        let mut scanner = SilentPaymentScanner::new(
            key_manager.scan_secret().clone(),
            *key_manager.spend_pubkey(),
        )?;
        
        // Add some test data
        let txid = Txid::from_str(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        )?;
        let outpoint = OutPoint::new(txid, 0);
        
        let payment_info = SilentPaymentInfo {
            txid,
            vout: 0,
            amount: 100000,
            block_height: Some(700000),
            spent: false,
        };
        
        scanner.add_detected_payment(outpoint, payment_info.clone());
        
        // Save scanner to file
        scanner.save_to_file(&file_path)?;
        
        // Verify file exists
        assert!(file_path.exists());
        
        // Load scanner from file
        let loaded_scanner = SilentPaymentScanner::load_from_file(
            &file_path,
            key_manager.scan_secret().clone(),
        )?;
        
        // Verify loaded data
        assert_eq!(loaded_scanner.scan_pubkey(), scanner.scan_pubkey());
        assert_eq!(loaded_scanner.spend_pubkey, scanner.spend_pubkey);
        
        // Verify payments
        assert_eq!(loaded_scanner.detected_payments.len(), 1);
        assert!(loaded_scanner.detected_payments.contains_key(&outpoint));
        
        let loaded_payment = loaded_scanner.detected_payments.get(&outpoint).unwrap();
        assert_eq!(loaded_payment.txid, payment_info.txid);
        assert_eq!(loaded_payment.vout, payment_info.vout);
        assert_eq!(loaded_payment.amount, payment_info.amount);
        assert_eq!(loaded_payment.block_height, payment_info.block_height);
        assert_eq!(loaded_payment.spent, payment_info.spent);
        
        // Clean up
        temp_dir.close()?;
        
        Ok(())
    }
    
    #[test]
    fn test_file_error_handling() -> anya_privacy::Result<()> {
        // Create a scanner
        let key_manager = KeyManager::new_random()?;
        let scanner = SilentPaymentScanner::new(
            key_manager.scan_secret().clone(),
            *key_manager.spend_pubkey(),
        )?;
        
        // Try to save to a non-existent directory with invalid permissions
        let invalid_path = PathBuf::from("/root/forbidden/path.json");
        
        // This should return an error
        let result = scanner.save_to_file(invalid_path);
        assert!(result.is_err());
        
        // Try to load from a non-existent file
        let non_existent_file = PathBuf::from("/tmp/non_existent_file.json");
        
        // Ensure the file doesn't exist
        if non_existent_file.exists() {
            fs::remove_file(&non_existent_file)?;
        }
        
        // This should return an error
        let result = SilentPaymentScanner::load_from_file(
            non_existent_file,
            key_manager.scan_secret().clone(),
        );
        assert!(result.is_err());
        
        Ok(())
    }
    
    #[test]
    fn test_cross_platform_paths() -> anya_privacy::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        
        // Test with platform-specific path separators
        #[cfg(target_os = "windows")]
        let file_path = temp_dir.path().join("test\\nested\\path.json");
        
        #[cfg(not(target_os = "windows"))]
        let file_path = temp_dir.path().join("test/nested/path.json");
        
        // Create a scanner
        let key_manager = KeyManager::new_random()?;
        let scanner = SilentPaymentScanner::new(
            key_manager.scan_secret().clone(),
            *key_manager.spend_pubkey(),
        )?;
        
        // Save scanner to file (should handle path creation)
        scanner.save_to_file(&file_path)?;
        
        // Verify file exists
        assert!(file_path.exists());
        
        // Clean up
        temp_dir.close()?;
        
        Ok(())
    }
}
