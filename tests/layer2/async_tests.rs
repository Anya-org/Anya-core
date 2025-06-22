//! Async Layer2 Protocol Tests
//! 
//! This module contains tests for the async implementations of Layer2 protocols

use anya_core::layer2::{
    AssetParams, AssetTransfer, BobClient, Layer2Protocol, Layer2ProtocolType,
    LightningNetwork, LiquidModule, RskClient, StacksClient, StateChannel, TaprootAssetsProtocol,
};

#[cfg(test)]
mod async_layer2_tests {
    use super::*;

    #[tokio::test]
    async fn test_bob_client_async_implementation() {
        // Test BobClient async implementation
        let bob_client = BobClient::default();
        
        // Test initialize
        let init_result = bob_client.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize BobClient: {:?}", init_result.err());
        
        // Test connect
        let connect_result = bob_client.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect BobClient: {:?}", connect_result.err());
        
        // Test get_state
        let state_result = bob_client.get_state().await;
        assert!(state_result.is_ok(), "Failed to get BobClient state: {:?}", state_result.err());
        
        // Test submit_transaction
        let tx_data = b"test transaction data";
        let submit_result = bob_client.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
        
        // Test transaction status
        let tx_id = submit_result.unwrap();
        let status_result = bob_client.check_transaction_status(&tx_id).await;
        assert!(status_result.is_ok(), "Failed to check transaction status: {:?}", status_result.err());
    }

    #[tokio::test]
    async fn test_liquid_module_async_implementation() {
        // Test LiquidModule async implementation
        let liquid_module = LiquidModule::default();
        
        // Test initialize
        let init_result = liquid_module.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize LiquidModule: {:?}", init_result.err());
        
        // Test connect
        let connect_result = liquid_module.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect LiquidModule: {:?}", connect_result.err());
        
        // Test get_state
        let state_result = liquid_module.get_state().await;
        assert!(state_result.is_ok(), "Failed to get LiquidModule state: {:?}", state_result.err());
        
        // Test submit_transaction
        let tx_data = b"test liquid transaction data";
        let submit_result = liquid_module.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
        
        // Test transaction status
        let tx_id = submit_result.unwrap();
        let status_result = liquid_module.check_transaction_status(&tx_id).await;
        assert!(status_result.is_ok(), "Failed to check transaction status: {:?}", status_result.err());
        
        // Test issue asset
        let asset_params = AssetParams {
            asset_id: "liquid_test_asset".to_string(),
            name: "Test Liquid Asset".to_string(),
            symbol: "TLA".to_string(),
            precision: 8,
            decimals: 8,
            total_supply: 1000000,
            metadata: "Test metadata".to_string(),
        };
        
        let issue_result = liquid_module.issue_asset(asset_params).await;
        assert!(issue_result.is_ok(), "Failed to issue asset: {:?}", issue_result.err());
    }

    #[tokio::test]
    async fn test_rsk_client_async_implementation() {
        // Test RskClient async implementation
        let rsk_client = RskClient::default();
        
        // Test initialize
        let init_result = rsk_client.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize RskClient: {:?}", init_result.err());
        
        // Test connect
        let connect_result = rsk_client.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect RskClient: {:?}", connect_result.err());
        
        // Test submit_transaction
        let tx_data = b"test rsk transaction data";
        let submit_result = rsk_client.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
    }

    #[tokio::test]
    async fn test_stacks_client_async_implementation() {
        // Test StacksClient async implementation
        let stacks_client = StacksClient::default();
        
        // Test initialize
        let init_result = stacks_client.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize StacksClient: {:?}", init_result.err());
        
        // Test connect
        let connect_result = stacks_client.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect StacksClient: {:?}", connect_result.err());
        
        // Test submit_transaction
        let tx_data = b"test stacks transaction data";
        let submit_result = stacks_client.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
    }

    #[tokio::test]
    async fn test_taproot_assets_protocol_async_implementation() {
        // Test TaprootAssetsProtocol async implementation
        let taproot_assets = TaprootAssetsProtocol::default();
        
        // Test initialize
        let init_result = taproot_assets.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize TaprootAssetsProtocol: {:?}", init_result.err());
        
        // Test connect
        let connect_result = taproot_assets.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect TaprootAssetsProtocol: {:?}", connect_result.err());
        
        // Test submit_transaction
        let tx_data = b"test taproot assets transaction data";
        let submit_result = taproot_assets.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
    }

    #[tokio::test]
    async fn test_lightning_network_async_implementation() {
        // Test LightningNetwork async implementation
        let lightning = LightningNetwork::default();
        
        // Test initialize
        let init_result = lightning.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize LightningNetwork: {:?}", init_result.err());
        
        // Test connect
        let connect_result = lightning.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect LightningNetwork: {:?}", connect_result.err());
        
        // Test submit_transaction
        let tx_data = b"test lightning transaction data";
        let submit_result = lightning.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
    }

    #[tokio::test]
    async fn test_state_channel_async_implementation() {
        // Test StateChannel async implementation
        let state_channel = StateChannel::default();
        
        // Test initialize
        let init_result = state_channel.initialize().await;
        assert!(init_result.is_ok(), "Failed to initialize StateChannel: {:?}", init_result.err());
        
        // Test connect
        let connect_result = state_channel.connect().await;
        assert!(connect_result.is_ok(), "Failed to connect StateChannel: {:?}", connect_result.err());
        
        // Test submit_transaction
        let tx_data = b"test state channel transaction data";
        let submit_result = state_channel.submit_transaction(tx_data).await;
        assert!(submit_result.is_ok(), "Failed to submit transaction: {:?}", submit_result.err());
    }
    
    #[tokio::test]
    async fn test_layer2_manager_async_initialization() {
        use anya_core::layer2::Layer2Manager;
        
        // Create a new Layer2Manager
        let mut manager = Layer2Manager::new();
        
        // Initialize all protocols asynchronously
        let init_result = manager.initialize_all_async().await;
        assert!(init_result.is_ok(), "Failed to initialize Layer2 protocols asynchronously: {:?}", init_result.err());
        
        // Check that all protocols are available through the async API
        let protocols = vec![
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            Layer2ProtocolType::RSK,
            Layer2ProtocolType::Stacks,
            Layer2ProtocolType::TaprootAssets,
            Layer2ProtocolType::Lightning,
            Layer2ProtocolType::StateChannels,
        ];
        
        for protocol_type in protocols {
            let protocol = manager.get_protocol_async(protocol_type);
            assert!(protocol.is_some(), "{:?} should be available after async initialization", protocol_type);
        }
    }
    
    #[tokio::test]
    async fn test_cross_protocol_transfers_async() {
        use anya_core::layer2::Layer2Manager;
        
        // Create a new Layer2Manager
        let mut manager = Layer2Manager::new();
        
        // Initialize all protocols asynchronously
        let init_result = manager.initialize_all_async().await;
        assert!(init_result.is_ok(), "Failed to initialize Layer2 protocols asynchronously");
        
        // Test cross-protocol transfers
        let transfer_id = manager.cross_layer_transfer(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset",
            2000,
        ).unwrap();
        
        assert!(transfer_id.contains("bob_liquid"));
    }

    #[tokio::test]
    async fn test_async_cross_layer_operations() {
        use anya_core::layer2::{Layer2Manager, Proof};
        
        // Create a new Layer2Manager
        let mut manager = Layer2Manager::new();
        
        // Initialize all protocols asynchronously
        let init_result = manager.initialize_all_async().await;
        assert!(init_result.is_ok(), "Failed to initialize Layer2 protocols asynchronously");
        
        // Test async cross-layer transfer
        let transfer_result = manager.cross_layer_transfer_async(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset",
            3000,
        ).await;
        
        assert!(transfer_result.is_ok(), "Failed to perform async cross-layer transfer");
        let transfer_id = transfer_result.unwrap();
        assert!(transfer_id.contains("bob_liquid"));
        
        // Test async proof verification
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
            ],
        ).await;
        
        assert!(verify_result.is_ok(), "Failed to verify cross-layer proof asynchronously");
        assert!(verify_result.unwrap(), "Cross-layer proof should be valid");
    }
}
