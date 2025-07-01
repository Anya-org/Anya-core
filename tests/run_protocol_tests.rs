use anya_core::layer2::{
    dlc::DlcProtocol,
    lightning::{LightningConfig, LightningNetwork},
    liquid::{LiquidConfig, LiquidModule},
    mock::MockLayer2Protocol,
    rgb::RgbProtocol,
    rsk::{RskClient, RskConfig},
    stacks::{StacksClient, StacksConfig},
    state_channels::StateChannelsProtocol,
    AssetParams, AssetTransfer, Layer2Protocol, Proof, TransactionStatus,
};

#[tokio::test]
async fn test_all_protocols() {
    // Create instances of all protocols
    let protocols: Vec<Box<dyn Layer2Protocol>> = vec![
        Box::new(LightningNetwork::new(LightningConfig::default())),
        Box::new(RgbProtocol::new()),
        Box::new(RskClient::new(RskConfig::default())),
        Box::new(DlcProtocol::new()),
        Box::new(StacksClient::new(StacksConfig::default())),
        Box::new(LiquidModule::new(LiquidConfig::default())),
        Box::new(StateChannelsProtocol::new()),
        Box::new(MockLayer2Protocol::new()),
    ];

    // Test each protocol
    for mut protocol in protocols {
        // Test initialization
        assert!(protocol.initialize().await.is_ok());

        // Test connection
        assert!(protocol.connect().await.is_ok());

        // Test transaction submission
        let tx_bytes = vec![1, 2, 3, 4];
        let tx_id = protocol.submit_transaction(&tx_bytes).await.unwrap();
        assert!(!tx_id.is_empty());

        // Test transaction status
        let status = protocol.check_transaction_status(&tx_id).await.unwrap();
        assert!(matches!(
            status,
            TransactionStatus::Confirmed | TransactionStatus::Pending
        ));

        // Test state management
        let state = protocol.get_state().await.unwrap();
        assert!(!state.version.is_empty());
        assert!(state.operational);

        assert!(protocol.sync_state().await.is_ok());

        // Test asset management
        let asset_params = AssetParams {
            asset_id: "test_asset_id".to_string(),
            name: "TestAsset".to_string(),
            symbol: "TST".to_string(),
            precision: 8,
            decimals: 8,
            total_supply: 1000000,
            metadata: "Test asset metadata".to_string(),
        };
        let asset_id = protocol.issue_asset(asset_params).await.unwrap();
        assert!(!asset_id.is_empty());

        let transfer = AssetTransfer {
            asset_id: asset_id.clone(),
            amount: 1000,
            from: "sender".to_string(),
            to: "receiver".to_string(),
            recipient: "receiver".to_string(),
            metadata: Some("Transfer metadata".to_string()),
        };
        let transfer_result = protocol.transfer_asset(transfer).await.unwrap();
        assert!(!transfer_result.tx_id.is_empty());
        assert!(matches!(
            transfer_result.status,
            TransactionStatus::Confirmed | TransactionStatus::Pending
        ));

        // Test proof verification
        let proof = Proof {
            proof_type: "merkle".to_string(),
            data: vec![1, 2, 3, 4],
            block_height: Some(12345),
            witness: Some(vec![5, 6, 7, 8]),
            merkle_root: "root".to_string(),
            merkle_proof: vec!["proof1".to_string(), "proof2".to_string()],
            block_header: "header".to_string(),
        };
        let verification_result = protocol.verify_proof(proof).await.unwrap();
        assert!(verification_result.is_valid);
        assert!(verification_result.error.is_none());

        // Test state validation
        let state_bytes = serde_json::to_vec(&state).unwrap();
        let validation_result = protocol.validate_state(&state_bytes).await.unwrap();
        assert!(validation_result.is_valid);
        assert!(validation_result.violations.is_empty());
    }
}
