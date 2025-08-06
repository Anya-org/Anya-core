#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_blockchain_anchoring_v1_3() -> Result<(), Box<dyn std::error::Error>> {
        // Create temporary directory for test registry
        let temp_dir = tempdir()?;
        let registry_path = temp_dir.path().join("test_registry.json");
        
        // Initialize registry
        let registry = SourceOfTruthRegistry::new(registry_path.to_string_lossy().to_string()).await?;
        
        // Enable anchoring and set network
        registry.enable_blockchain_anchoring();
        registry.set_bitcoin_network("testnet")?;
        
        // Verify network setting
        assert_eq!(registry.get_bitcoin_network(), "testnet");
        
        // Create an anchor
        let anchor = registry.anchor_to_blockchain().await?;
        
        // Verify anchor was created with correct initial state
        assert!(registry.blockchain_anchors.contains_key(&anchor.txid));
        assert_eq!(anchor.network, "testnet");
        assert!(matches!(anchor.status, AnchorStatus::Created));
        
        // Sync with blockchain (this will simulate progression through states)
        let updated_txids = registry.sync_with_blockchain().await?;
        assert!(updated_txids.contains(&anchor.txid));
        
        // Get updated anchor and verify state changed
        let updated_anchor = registry.blockchain_anchors.get(&anchor.txid).unwrap();
        assert!(matches!(updated_anchor.status, AnchorStatus::Broadcast));
        
        // Run another sync
        let updated_txids = registry.sync_with_blockchain().await?;
        assert!(updated_txids.contains(&anchor.txid));
        
        // Get updated anchor and verify state changed to Confirmed
        let updated_anchor = registry.blockchain_anchors.get(&anchor.txid).unwrap();
        assert!(if let AnchorStatus::Confirmed(_) = updated_anchor.status { true } else { false });
        
        Ok(())
    }

    #[cfg(feature = "taproot")]
    #[tokio::test]
    async fn test_taproot_anchoring() -> Result<(), Box<dyn std::error::Error>> {
        // Create temporary directory for test registry
        let temp_dir = tempdir()?;
        let registry_path = temp_dir.path().join("test_registry.json");
        
        // Initialize registry
        let registry = SourceOfTruthRegistry::new(registry_path.to_string_lossy().to_string()).await?;
        
        // Enable anchoring and set network
        registry.enable_blockchain_anchoring();
        
        // Create internal key and script tree (dummy values for test)
        let internal_key = vec![0; 32];
        let script_tree_hashes = vec![[0; 32]];
        
        // Create Taproot anchor
        let anchor = registry.create_taproot_anchor(
            internal_key.clone(),
            script_tree_hashes.clone(),
            "testnet"
        ).await?;
        
        // Verify Taproot-specific data
        assert!(registry.blockchain_anchors.contains_key(&anchor.txid));
        assert!(anchor.taproot_data.is_some());
        
        let taproot_data = anchor.taproot_data.unwrap();
        assert_eq!(taproot_data.internal_key, internal_key);
        assert_eq!(taproot_data.script_tree_hashes, script_tree_hashes);
        
        Ok(())
    }
}
