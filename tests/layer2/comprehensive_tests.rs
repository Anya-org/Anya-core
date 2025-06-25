//! Layer2 Protocol Comprehensive Test Suite
//! 
//! This file contains comprehensive test coverage for all Layer2 protocol implementations.

use anya_core::layer2::{
    BobClient, Layer2ProtocolTrait, LightningNetwork, LiquidModule, RskClient, StacksClient, 
    StateChannel, TaprootAssetsProtocol, Layer2Manager, Layer2ProtocolType,
    AssetParams, AssetTransfer, Proof, ProtocolState, TransactionStatus,
    TransferResult, ValidationResult, VerificationResult,
};

#[cfg(test)]
mod layer2_protocol_tests {
    use super::*;

    // Test Default trait implementation for all Layer2 protocol clients
    #[test]
    fn test_all_layer2_default_implementations() {
        // Test BobClient default implementation
        let bob_client = BobClient::default();
        assert_eq!(bob_client.get_version(), "0.1.0");
        
        // Test LiquidModule default implementation
        let liquid_module = LiquidModule::default();
        assert_eq!(liquid_module.get_network(), "mainnet");
        
        // Test RskClient default implementation
        let rsk_client = RskClient::default();
        assert_eq!(rsk_client.get_network(), "mainnet");
        
        // Test StacksClient default implementation
        let stacks_client = StacksClient::default();
        assert_eq!(stacks_client.get_network(), "mainnet");
        
        // Test TaprootAssetsProtocol default implementation
        let taproot_assets = TaprootAssetsProtocol::default();
        assert_eq!(taproot_assets.get_version(), "0.1.0");
        
        // Test LightningNetwork default implementation
        let lightning = LightningNetwork::default();
        assert!(lightning.list_channels().is_empty());
        
        // Test StateChannel default implementation
        let state_channel = StateChannel::default();
        assert_eq!(state_channel.state, crate::layer2::state_channels::ChannelState::Creating);
    }
    
    // Test Layer2Manager initialization with all protocols
    #[test]
    fn test_layer2_manager_initialization() {
        let mut manager = Layer2Manager::new();
        
        // Initialize all protocols
        let result = manager.initialize_all();
        assert!(result.is_ok(), "Failed to initialize Layer2 protocols: {:?}", result.err());
        
        // Verify each protocol is available
        assert!(manager.get_protocol(Layer2ProtocolType::BOB).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::Liquid).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::RSK).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::Stacks).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::TaprootAssets).is_some());
    }
    
    // Test cross-layer transfers
    #[test]
    fn test_cross_layer_transfers() {
        let mut manager = Layer2Manager::new();
        manager.initialize_all().unwrap();
        
        // Test transfers between different Layer2 protocols
        let transfer_result = manager.cross_layer_transfer(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset_id",
            1000,
        );
        
        assert!(transfer_result.is_ok());
        let transfer_id = transfer_result.unwrap();
        assert!(transfer_id.contains("bob_liquid"));
    }
    
    // Test protocol state retrieval
    #[test]
    fn test_protocol_state_retrieval() {
        // Test BobClient protocol state
        let bob_client = BobClient::default();
        let state_result = bob_client.get_state();
        assert!(state_result.is_ok());
        let state = state_result.unwrap();
        assert!(state.operational);
        
        // Test LiquidModule protocol state
        let liquid_module = LiquidModule::default();
        let state_result = liquid_module.get_state();
        assert!(state_result.is_ok());
        let state = state_result.unwrap();
        assert!(state.operational);
    }
}
