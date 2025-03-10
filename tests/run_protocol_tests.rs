use anya_core::layer2::{
    Layer2Protocol,
    ProtocolState,
    TransactionStatus,
    AssetParams,
    AssetTransfer,
    TransferResult,
    Proof,
    VerificationResult,
    ValidationResult,
    bob::BobClient,
    lightning::LightningProtocol,
    rgb::RgbProtocol,
    rsk::RskProtocol,
    dlc::DlcProtocol,
    stacks::StacksProtocol,
    liquid::LiquidProtocol,
    state_channels::StateChannelsProtocol,
    mock::MockLayer2Protocol,
};

#[tokio::test]
async fn test_all_protocols() {
    // Create instances of all protocols
    let protocols: Vec<Box<dyn Layer2Protocol>> = vec![
        Box::new(LightningProtocol::new()),
        Box::new(RgbProtocol::new()),
        Box::new(RskProtocol::new()),
        Box::new(DlcProtocol::new()),
        Box::new(StacksProtocol::new()),
        Box::new(LiquidProtocol::new()),
        Box::new(StateChannelsProtocol::new()),
        Box::new(MockLayer2Protocol::new()),
    ];

    // Test each protocol
    for protocol in protocols {
        // Test initialization
        assert!(protocol.initialize().await.is_ok());

        // Test connection
        assert!(protocol.connect().await.is_ok());

        // Test transaction submission
        let tx_bytes = vec![1, 2, 3, 4];
        let tx_id = protocol.submit_transaction(&tx_bytes).await.unwrap();
        assert!(!tx_id.is_empty());

        // Test transaction status
        let status = protocol.get_transaction_status(&tx_id).await.unwrap();
        assert!(matches!(status, TransactionStatus::Confirmed | TransactionStatus::Pending));

        // Test state management
        let state = protocol.get_state().await.unwrap();
        assert!(state.height >= 0);
        assert!(!state.hash.is_empty());
        assert!(state.timestamp >= 0);

        assert!(protocol.sync_state().await.is_ok());

        // Test asset management
        let asset_params = AssetParams {
            name: "TestAsset".to_string(),
            symbol: "TEST".to_string(),
            decimals: 8,
            total_supply: 1000000,
        };
        let asset_id = protocol.issue_asset(asset_params).await.unwrap();
        assert!(!asset_id.is_empty());

        let transfer = AssetTransfer {
            asset_id: asset_id.clone(),
            amount: 1000,
            from: "sender".to_string(),
            to: "receiver".to_string(),
        };
        let transfer_result = protocol.transfer_asset(transfer).await.unwrap();
        assert!(!transfer_result.tx_id.is_empty());
        assert!(matches!(transfer_result.status, TransactionStatus::Confirmed | TransactionStatus::Pending));
        assert!(transfer_result.timestamp >= 0);

        // Test proof verification
        let proof = Proof {
            merkle_root: "root".to_string(),
            merkle_proof: vec!["proof1".to_string(), "proof2".to_string()],
            block_header: "header".to_string(),
        };
        let verification_result = protocol.verify_proof(&proof).await.unwrap();
        assert!(verification_result.valid);
        assert!(verification_result.error.is_none());

        // Test state validation
        let validation_result = protocol.validate_state(&state).await.unwrap();
        assert!(validation_result.valid);
        assert!(validation_result.error.is_none());

        // Test disconnection
        assert!(protocol.disconnect().await.is_ok());
    }
} 