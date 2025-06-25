//! Comprehensive Async Layer2 Protocol Tests
//! 
//! This module contains comprehensive tests for all async Layer2 protocol implementations,
//! including edge cases and more complex scenarios.

use anya_core::layer2::{
    AssetParams, AssetTransfer, BobClient, Layer2Protocol, Layer2ProtocolType, Layer2Manager,
    LightningNetwork, LiquidModule, Proof, RskClient, StacksClient, 
    StateChannel, TaprootAssetsProtocol, TransactionStatus, ValidationResult, VerificationResult,
};

#[cfg(test)]
mod comprehensive_async_layer2_tests {
    use super::*;
    use std::collections::HashMap;

    // Test helper to verify all fundamental async trait methods for a Layer2Protocol
    async fn test_protocol_async_basics<T: Layer2Protocol>(protocol: &mut T, protocol_name: &str) {
        println!("Testing basic async implementation for {}", protocol_name);
        
        // Test initialize
        let init_result = protocol.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize {}: {:?}", protocol_name, init_result.err());
        
        // Test connect
        let connect_result = protocol.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect {}: {:?}", protocol_name, connect_result.err());
        
        // Test get_state
        let state_result = protocol.get_state().await;
        assert!(state_result.is_ok(), "Failed to get {} state: {:?}", protocol_name, state_result.err());
        
        let state = state_result.unwrap();
        println!("{} state: {:?}", protocol_name, state);
        
        // Test submit_transaction
        let tx_data = format!("test {} transaction data", protocol_name).into_bytes();
        let submit_result = protocol.submit_transaction(&tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit {} transaction: {:?}", protocol_name, submit_result.err());
        
        let tx_id = submit_result.unwrap();
        
        // Test check_transaction_status
        let status_result = protocol.check_transaction_status(&tx_id).await;
        assert!(status_result.is_ok(), "Failed to check {} transaction status: {:?}", protocol_name, status_result.err());
        
        // Test sync_state
        let sync_result = protocol.sync_state().await;
        assert!(sync_result.is_ok(), "Failed to sync {} state: {:?}", protocol_name, sync_result.err());
    }

    #[tokio::test]
    async fn test_all_protocols_async_basics() {
        // Test each protocol's async implementation
        let mut bob_client = BobClient::default();
        let mut liquid_module = LiquidModule::default();
        let mut rsk_client = RskClient::default();
        let mut stacks_client = StacksClient::default();
        let mut taproot_assets = TaprootAssetsProtocol::default();
        let mut lightning_network = LightningNetwork::default();
        let mut state_channel = StateChannel::default();
        
        // Test each protocol's basic async functionality
        test_protocol_async_basics(&mut bob_client, "BobClient").await;
        test_protocol_async_basics(&mut liquid_module, "LiquidModule").await;
        test_protocol_async_basics(&mut rsk_client, "RskClient").await;
        test_protocol_async_basics(&mut stacks_client, "StacksClient").await;
        test_protocol_async_basics(&mut taproot_assets, "TaprootAssetsProtocol").await;
        test_protocol_async_basics(&mut lightning_network, "LightningNetwork").await;
        test_protocol_async_basics(&mut state_channel, "StateChannel").await;
    }
    
    #[tokio::test]
    async fn test_asset_operations_async() {
        // Test asset operations using async API
        let test_protocols = vec![
            ("BobClient", BobClient::default() as Box<dyn Layer2Protocol>),
            ("LiquidModule", LiquidModule::default() as Box<dyn Layer2Protocol>),
            // Skip LightningNetwork because it doesn't support asset operations
            // Skip StateChannel because it doesn't support asset operations
        ];
        
        for (protocol_name, mut protocol) in test_protocols {
            if protocol_name.contains("Lightning") || protocol_name.contains("StateChannel") {
                // Skip protocols that don't support asset operations
                continue;
            }
            
            println!("Testing asset operations for {}", protocol_name);
            
            // Initialize protocol
            protocol.initialize().await.expect(&format!("Failed to initialize {}", protocol_name));
            
            // Create asset parameters
            let asset_params = AssetParams {
                asset_id: format!("{}_test_asset", protocol_name.to_lowercase()),
                name: format!("{} Test Asset", protocol_name),
                symbol: format!("{}TA", protocol_name.chars().next().unwrap()),
                precision: 8,
                decimals: 8,
                total_supply: 1000000,
                metadata: format!("{} test metadata", protocol_name),
            };
            
            // Issue asset
            let issue_result = protocol.issue_asset(asset_params.clone()).await;
            assert!(issue_result.is_ok(), "Failed to issue asset on {}: {:?}", protocol_name, issue_result.err());
            
            let asset_id = issue_result.unwrap();
            println!("{} issued asset: {}", protocol_name, asset_id);
            
            // Create asset transfer
            let transfer = AssetTransfer {
                asset_id: asset_id.clone(),
                amount: 1000,
                from: "sender_address".to_string(),
                to: "recipient_address".to_string(),
                recipient: "recipient_address".to_string(),
                metadata: Some("Transfer test".to_string()),
            };
            
            // Transfer asset
            let transfer_result = protocol.transfer_asset(transfer).await;
            assert!(transfer_result.is_ok(), "Failed to transfer asset on {}: {:?}", protocol_name, transfer_result.err());
            
            let result = transfer_result.unwrap();
            println!("{} transfer result: {:?}", protocol_name, result);
            assert!(result.tx_id.contains(&protocol_name.to_lowercase()) || result.tx_id.contains(&asset_id));
        }
    }
    
    #[tokio::test]
    async fn test_proof_verification_async() {
        // Test proof verification using async API
        let test_protocols = vec![
            ("BobClient", BobClient::default() as Box<dyn Layer2Protocol>),
            ("LiquidModule", LiquidModule::default() as Box<dyn Layer2Protocol>),
            ("RskClient", RskClient::default() as Box<dyn Layer2Protocol>),
            ("StacksClient", StacksClient::default() as Box<dyn Layer2Protocol>),
            ("TaprootAssetsProtocol", TaprootAssetsProtocol::default() as Box<dyn Layer2Protocol>),
            ("LightningNetwork", LightningNetwork::default() as Box<dyn Layer2Protocol>),
            ("StateChannel", StateChannel::default() as Box<dyn Layer2Protocol>),
        ];
        
        // Create a test proof
        let proof = Proof {
            proof_type: "test_proof".to_string(),
            data: vec![1, 2, 3, 4, 5],
            block_height: Some(12345),
            witness: Some(vec![10, 20, 30]),
            merkle_root: "test_merkle_root".to_string(),
            merkle_proof: vec!["proof_1".to_string(), "proof_2".to_string()],
            block_header: "test_block_header".to_string(),
        };
        
        for (protocol_name, protocol) in test_protocols {
            println!("Testing proof verification for {}", protocol_name);
            
            // Initialize protocol
            protocol.initialize().await.expect(&format!("Failed to initialize {}", protocol_name));
            
            // Verify proof
            let verify_result = protocol.verify_proof(proof.clone()).await;
            assert!(verify_result.is_ok(), "Failed to verify proof on {}: {:?}", protocol_name, verify_result.err());
            
            let result = verify_result.unwrap();
            println!("{} verification result: {:?}", protocol_name, result);
        }
    }
    
    #[tokio::test]
    async fn test_layer2_manager_comprehensive_async() {
        // Create a Layer2Manager instance
        let mut manager = Layer2Manager::new();
        
        // Initialize asynchronously
        let init_result = manager.initialize_all_async().await;
        assert!(init_result.is_ok(), "Failed to initialize Layer2Manager asynchronously: {:?}", init_result.err());
        
        // Test all protocols are accessible
        let protocol_types = vec![
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            Layer2ProtocolType::RSK,
            Layer2ProtocolType::Stacks,
            Layer2ProtocolType::TaprootAssets,
            Layer2ProtocolType::Lightning,
            Layer2ProtocolType::StateChannels,
        ];
        
        for protocol_type in protocol_types {
            let protocol = manager.get_protocol_async(protocol_type);
            assert!(protocol.is_some(), "Protocol {:?} should be available", protocol_type);
        }
        
        // Test cross-layer transfer (async)
        let transfer_result = manager.cross_layer_transfer_async(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset",
            5000,
        ).await;
        
        assert!(transfer_result.is_ok(), "Failed to perform async cross-layer transfer: {:?}", transfer_result.err());
        let transfer_id = transfer_result.unwrap();
        assert!(transfer_id.contains("bob_liquid"));
        
        // Test sync protocol interfaces still work after async initialization
        let bob_sync = manager.get_protocol(Layer2ProtocolType::BOB);
        assert!(bob_sync.is_some(), "BOB protocol should be available via sync interface");
        
        // Test proof verification across protocols (async)
        let proof = Proof {
            proof_type: "merkle".to_string(),
            data: vec![1, 2, 3, 4, 5],
            block_height: Some(100),
            witness: Some(vec![10, 11, 12]),
            merkle_root: "test_root".to_string(),
            merkle_proof: vec!["proof1".to_string(), "proof2".to_string()],
            block_header: "test_header".to_string(),
        };
        
        let verify_result = manager.verify_cross_layer_proof_async(
            proof,
            vec![
                Layer2ProtocolType::BOB,
                Layer2ProtocolType::Liquid,
                Layer2ProtocolType::RSK,
                Layer2ProtocolType::Stacks,
            ],
        ).await;
        
        assert!(verify_result.is_ok(), "Failed to verify cross-layer proof asynchronously: {:?}", verify_result.err());
        assert!(verify_result.unwrap(), "Cross-layer proof should be valid");
    }
}
