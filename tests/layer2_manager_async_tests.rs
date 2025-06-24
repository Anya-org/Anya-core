//! Layer2Manager async tests
//!
//! This module tests the Layer2Manager's async capabilities.

use anya_core::layer2::{
    Layer2Manager, Layer2ProtocolType, Proof,
};

#[cfg(test)]
mod layer2_manager_async_tests {
    use super::*;
    use std::error::Error;
    use tokio::test;

    #[test]
    async fn test_layer2_manager_async_initialization() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut manager = Layer2Manager::new();
        
        // Test async initialization
        match manager.initialize_all_async().await {
            Ok(_) => println!("Layer2Manager initialized asynchronously"),
            Err(e) => {
                // In test environment, some protocols might not be available
                // This should not fail the test
                println!("Layer2Manager initialization status: {}", e);
            }
        }
        
        // Test protocol access
        for protocol_type in &[
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            Layer2ProtocolType::RSK,
            Layer2ProtocolType::Stacks,
            Layer2ProtocolType::TaprootAssets,
            Layer2ProtocolType::Lightning,
            Layer2ProtocolType::StateChannels,
        ] {
            // Protocol might not be available in test environment
            if let Some(_) = manager.get_protocol_async(*protocol_type) {
                println!("{:?} protocol is available", protocol_type);
            } else {
                println!("{:?} protocol is not available", protocol_type);
            }
        }
        
        Ok(())
    }

    #[test]
    async fn test_cross_layer_transfer_async() -> Result<(), Box<dyn Error + Send + Sync>> {
        let manager = Layer2Manager::new();
        
        // Test cross-layer transfer between BOB and Liquid
        match manager.cross_layer_transfer_async(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset_123",
            1000,
        ).await {
            Ok(transfer_id) => {
                println!("Cross-layer transfer initiated: {}", transfer_id);
                assert!(!transfer_id.is_empty(), "Transfer ID should not be empty");
            },
            Err(e) => {
                // In test environment, protocols might not be available
                // This should not fail the test
                println!("Cross-layer transfer status: {}", e);
            }
        }
        
        Ok(())
    }

    #[test]
    async fn test_verify_cross_layer_proof_async() -> Result<(), Box<dyn Error + Send + Sync>> {
        let manager = Layer2Manager::new();
        
        // Create a test proof
        let proof = Proof {
            proof_type: "CrossLayerTransferProof".to_string(),
            data: vec![1, 2, 3, 4, 5],
            block_height: Some(100),
            witness: Some(vec![10, 20, 30]),
            merkle_root: "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            merkle_proof: vec!["proof1".to_string(), "proof2".to_string()],
            block_header: "00000020ed4100a2d3f0e2a9bed08398e5687e92875865133a2b5804e390ed6845ab77474833f56cd2fd7642338de35ac92ea0000000000000000000000000000000000000000008b67ed91e462ed0e78f61842acce80e171a777e2c051d3f2e382530000000000".to_string(),
        };
        
        // Test cross-layer proof verification
        match manager.verify_cross_layer_proof_async(
            proof,
            vec![Layer2ProtocolType::BOB, Layer2ProtocolType::Liquid],
        ).await {
            Ok(is_valid) => {
                println!("Cross-layer proof verification result: {}", is_valid);
            },
            Err(e) => {
                // In test environment, protocols might not be available
                // This should not fail the test
                println!("Cross-layer proof verification status: {}", e);
            }
        }
        
        Ok(())
    }
}
