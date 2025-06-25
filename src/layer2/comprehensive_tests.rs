#[cfg(test)]
mod comprehensive_layer2_tests {
    // Removed unused import
    use crate::layer2::{
        AssetParams, AssetTransfer, BobClient, Layer2ProtocolType,
        Layer2ProtocolTrait, LiquidModule, Proof, RskClient, StacksClient, TaprootAssetsProtocol,
    };

    #[test]
    fn test_bob_client_integration() {
        let config = crate::layer2::bob::BobConfig::default();
        let client = BobClient::new(config);

        // Test initialization
        assert!(client.initialize().is_ok());

        // Test state retrieval
        let state = client.get_state().unwrap();
        assert_eq!(state.version, "1.0.0");

        // Test transaction submission
        let tx_data = b"test_transaction";
        let tx_id = client.submit_transaction(tx_data).unwrap();
        assert!(tx_id.starts_with("bob_tx_"));

        // Test asset issuance
        let asset_params = AssetParams {
            asset_id: "test_asset".to_string(),
            name: "Test Asset".to_string(),
            symbol: "TST".to_string(),
            precision: 8,
            decimals: 8,
            total_supply: 1000000,
            metadata: "test metadata".to_string(),
        };
        let asset_id = client.issue_asset(asset_params).unwrap();
        assert!(asset_id.starts_with("bob_asset_"));
    }

    #[test]
    fn test_liquid_module_integration() {
        // Create a fresh configuration and module instance
        let config = crate::layer2::liquid::LiquidConfig::default();
        let module = LiquidModule::new(config);

        // Test initialization
        assert!(module.initialize().is_ok(), "Module initialization failed");

        // Test state retrieval and make sure it matches the expected version
        match module.get_state() {
            Ok(state) => {
                // Verify the exact version from LiquidModule::new
                assert_eq!(state.version, "23.2.1", "Version mismatch: expected '23.2.1', got '{}'", state.version);
                assert_eq!(state.capacity, Some(21000000), "Capacity mismatch");
            },
            Err(e) => {
                panic!("Failed to get state: {:?}", e);
            }
        }

        // Test transaction submission
        let tx_data = b"liquid_transaction";
        let tx_id = module.submit_transaction(tx_data).unwrap();
        assert!(tx_id.starts_with("liquid_tx_"), "Transaction ID format incorrect");

        // Test asset issuance
        let asset_params = AssetParams {
            asset_id: "liquid_asset".to_string(),
            name: "Liquid Test Asset".to_string(),
            symbol: "LTA".to_string(),
            precision: 8,
            decimals: 8,
            total_supply: 500000,
            metadata: "liquid metadata".to_string(),
        };
        let asset_id = module.issue_asset(asset_params).unwrap();
        assert!(asset_id.starts_with("liquid_asset_"), "Asset ID format incorrect");
    }

    #[test]
    fn test_rsk_client_integration() {
        let config = crate::layer2::rsk::RskConfig::default();
        let client = RskClient::new(config);

        // Test initialization
        assert!(client.initialize().is_ok());

        // Test state retrieval
        let state = client.get_state().unwrap();
        assert_eq!(state.version, "1.0.0");
        assert_eq!(state.capacity, Some(21000000)); // RBTC supply

        // Test smart contract deployment
        let bytecode = b"smart_contract_bytecode";
        let contract_address = client.deploy_contract(bytecode).unwrap();
        assert!(contract_address.starts_with("rsk_contract_"));

        // Test asset transfer
        let transfer = AssetTransfer {
            asset_id: "rsk_asset".to_string(),
            amount: 1000,
            from: "rsk_sender".to_string(),
            to: "rsk_recipient".to_string(),
            recipient: "rsk_recipient".to_string(),
            metadata: Some("rsk transfer".to_string()),
        };
        let result = client.transfer_asset(transfer).unwrap();
        assert!(result.tx_id.starts_with("rsk_transfer_"));
        assert_eq!(result.fee, Some(500));
    }

    #[test]
    fn test_stacks_client_integration() {
        let config = crate::layer2::stacks::StacksConfig::default();
        let client = StacksClient::new(config);

        // Test initialization
        assert!(client.initialize().is_ok());

        // Test state retrieval
        let state = client.get_state().unwrap();
        assert_eq!(state.version, "2.0.0"); // Stacks 2.0
        assert_eq!(state.capacity, Some(1320000000)); // STX supply

        // Test Clarity contract deployment
        let contract_code = "(define-public (hello) (ok \"Hello World\"))";
        let contract_id = client
            .deploy_clarity_contract(contract_code, "hello-world")
            .unwrap();
        assert!(contract_id.starts_with("stacks_contract_"));

        // Test contract function call
        let call_result = client
            .call_contract_function("hello-world", "hello", vec![])
            .unwrap();
        assert!(call_result.starts_with("stacks_call_"));

        // Test SIP-010 token issuance
        let asset_params = AssetParams {
            asset_id: "stx_token".to_string(),
            name: "STX Test Token".to_string(),
            symbol: "STX".to_string(),
            precision: 6,
            decimals: 6,
            total_supply: 10000000,
            metadata: "SIP-010 token".to_string(),
        };
        let token_id = client.issue_asset(asset_params).unwrap();
        assert!(token_id.starts_with("stacks_token_"));
    }

    #[test]
    fn test_taproot_assets_integration() {
        let config = crate::layer2::taproot_assets::TaprootAssetsConfig::default();
        let protocol = TaprootAssetsProtocol::new(config);

        // Test initialization
        assert!(protocol.initialize().is_ok());

        // Test state retrieval
        let state = protocol.get_state().unwrap();
        assert_eq!(state.version, "0.3.0"); // Current Taproot Assets version
        assert_eq!(state.capacity, None); // No fixed capacity

        // Test asset minting
        let asset_id = protocol
            .mint_asset("Test Asset", 1000000, "normal")
            .unwrap();
        assert!(asset_id.starts_with("taproot_asset_"));

        // Test universe proof creation
        let proof_data = protocol.create_universe_proof("test_asset").unwrap();
        assert!(!proof_data.is_empty());

        // Test Merkle proof verification
        let merkle_proof = Proof {
            proof_type: "merkle".to_string(),
            data: vec![0x01, 0x02, 0x03],
            block_height: Some(800000),
            witness: None,
            merkle_root: "root_hash".to_string(),
            merkle_proof: vec!["proof1".to_string(), "proof2".to_string()],
            block_header: "header_data".to_string(),
        };
        let verification = protocol.verify_proof(merkle_proof).unwrap();
        assert!(verification.is_valid);
        assert!(verification.error.is_none());

        // Test invalid proof type
        let invalid_proof = Proof {
            proof_type: "invalid".to_string(),
            data: vec![],
            block_height: None,
            witness: None,
            merkle_root: "invalid_root".to_string(),
            merkle_proof: vec![],
            block_header: "invalid_header".to_string(),
        };
        let invalid_verification = protocol.verify_proof(invalid_proof).unwrap();
        assert!(!invalid_verification.is_valid);
        assert!(invalid_verification.error.is_some());
    }

    #[test]
    fn test_cross_protocol_compatibility() {
        // Initialize all protocols
        let bob_client = BobClient::new(Default::default());
        let liquid_module = LiquidModule::new(Default::default());
        let rsk_client = RskClient::new(Default::default());
        let stacks_client = StacksClient::new(Default::default());
        let taproot_assets = TaprootAssetsProtocol::new(Default::default());

        // Test that all protocols can be initialized
        assert!(bob_client.initialize().is_ok());
        assert!(liquid_module.initialize().is_ok());
        assert!(rsk_client.initialize().is_ok());
        assert!(stacks_client.initialize().is_ok());
        assert!(taproot_assets.initialize().is_ok());

        // Test that all protocols can submit transactions
        let test_data = b"cross_protocol_test";

        assert!(bob_client.submit_transaction(test_data).is_ok());
        assert!(liquid_module.submit_transaction(test_data).is_ok());
        assert!(rsk_client.submit_transaction(test_data).is_ok());
        assert!(stacks_client.submit_transaction(test_data).is_ok());
        assert!(taproot_assets.submit_transaction(test_data).is_ok());
    }

    /* 
    // Commented out manager tests since manager module is not available
    #[test]
    fn test_layer2_manager_comprehensive() {
        let mut manager = Layer2Manager::new();

        // Test initialization of all protocols
        assert!(manager.initialize_all().is_ok());

        // Test protocol availability
        assert!(manager.get_protocol(Layer2ProtocolType::BOB).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::Liquid).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::RSK).is_some());
        assert!(manager.get_protocol(Layer2ProtocolType::Stacks).is_some());
        assert!(manager
            .get_protocol(Layer2ProtocolType::TaprootAssets)
            .is_some());

        // Test cross-layer transfers between different protocols
        let bob_to_liquid = manager.cross_layer_transfer(
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            "test_asset",
            1000,
        );
        assert!(bob_to_liquid.is_ok());

        let stacks_to_rsk = manager.cross_layer_transfer(
            Layer2ProtocolType::Stacks,
            Layer2ProtocolType::RSK,
            "another_asset",
            500,
        );
        assert!(stacks_to_rsk.is_ok());

        let taproot_to_bob = manager.cross_layer_transfer(
            Layer2ProtocolType::TaprootAssets,
            Layer2ProtocolType::BOB,
            "taproot_asset",
            2000,
        );
        assert!(taproot_to_bob.is_ok());
    }
    */

    #[test]
    fn test_all_protocol_enum_variants() {
        // Ensure all protocol enum variants are properly defined
        let protocols = vec![
            Layer2ProtocolType::Lightning,
            Layer2ProtocolType::StateChannels,
            Layer2ProtocolType::RGB,
            Layer2ProtocolType::DLC,
            Layer2ProtocolType::BOB,
            Layer2ProtocolType::Liquid,
            Layer2ProtocolType::RSK,
            Layer2ProtocolType::Stacks,
            Layer2ProtocolType::TaprootAssets,
        ];

        // Test serialization/deserialization of all protocol types
        for protocol in protocols {
            let serialized = serde_json::to_string(&protocol).unwrap();
            let _deserialized: Layer2ProtocolType = serde_json::from_str(&serialized).unwrap();
        }
    }

    #[test]
    fn test_protocol_state_validation() {
        // Test that all protocols return valid state information
        let bob_client = BobClient::new(Default::default());
        let liquid_module = LiquidModule::new(Default::default());
        let rsk_client = RskClient::new(Default::default());
        let stacks_client = StacksClient::new(Default::default());
        let taproot_assets = TaprootAssetsProtocol::new(Default::default());

        // Initialize all protocols
        bob_client.initialize().unwrap();
        liquid_module.initialize().unwrap();
        rsk_client.initialize().unwrap();
        stacks_client.initialize().unwrap();
        taproot_assets.initialize().unwrap();

        // Test state validation with empty data
        let empty_state = b"";

        let bob_validation = bob_client.validate_state(empty_state).unwrap();
        assert!(bob_validation.is_valid);

        let liquid_validation = liquid_module.validate_state(empty_state).unwrap();
        assert!(liquid_validation.is_valid);

        let rsk_validation = rsk_client.validate_state(empty_state).unwrap();
        assert!(rsk_validation.is_valid);

        let stacks_validation = stacks_client.validate_state(empty_state).unwrap();
        assert!(stacks_validation.is_valid);

        let taproot_validation = taproot_assets.validate_state(empty_state).unwrap();
        assert!(!taproot_validation.is_valid); // Taproot Assets requires non-empty state
        assert!(!taproot_validation.violations.is_empty());
    }
}
