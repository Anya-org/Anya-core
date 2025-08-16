//! Working Layer2 Integration Tests
//!
//! Tests basic functionality of implemented Layer2 protocols using actual working implementations.

use anya_core::layer2::lightning::LightningConfig;
use anya_core::layer2::{
    AssetParams, AssetTransfer, BobProtocol, DlcProtocol, Layer2Protocol, LightningProtocol,
    RgbProtocol, StateChannelsProtocol, TransactionStatus,
};

/// Test all Layer 2 protocols basic functionality
#[tokio::test]
async fn test_layer2_protocols_basic_operations() {
    // Test Lightning Network
    let lightning = LightningProtocol::default();
    assert!(lightning.initialize().await.is_ok());
    assert!(lightning.connect().await.is_ok());

    let lightning_state = lightning.get_state().await.unwrap();
    assert!(!lightning_state.version.is_empty());

    // Test RGB Protocol
    let rgb = RgbProtocol::default();
    assert!(rgb.initialize().await.is_ok());
    assert!(rgb.connect().await.is_ok());

    let rgb_state = rgb.get_state().await.unwrap();
    assert!(!rgb_state.version.is_empty());

    // Test DLC Protocol
    let dlc = DlcProtocol::default();
    assert!(dlc.initialize().await.is_ok());
    assert!(dlc.connect().await.is_ok());

    let dlc_state = dlc.get_state().await.unwrap();
    assert!(!dlc_state.version.is_empty());

    // Test State Channels
    let state_channels = StateChannelsProtocol::default();
    assert!(state_channels.initialize().await.is_ok());
    assert!(state_channels.connect().await.is_ok());

    let sc_state = state_channels.get_state().await.unwrap();
    assert!(!sc_state.version.is_empty());

    // Test BOB Protocol
    let bob = BobProtocol::new();
    assert!(bob.initialize().await.is_ok());
    assert!(bob.connect().await.is_ok());

    println!("All Layer 2 protocols initialized successfully");
}

/// Test asset operations on RGB protocol
#[tokio::test]
async fn test_rgb_asset_operations() {
    let rgb = RgbProtocol::default();

    // Initialize and connect
    rgb.initialize().await.unwrap();
    rgb.connect().await.unwrap();

    // Test asset issuance using the Layer2Protocol trait method
    let asset_params = AssetParams {
        name: "Test Asset".to_string(),
        symbol: "TEST".to_string(),
        total_supply: 1_000_000,
        metadata: "Test asset for RGB protocol".to_string(),
    };

    let asset_id = rgb.issue_asset(asset_params).await.unwrap();
    assert!(!asset_id.is_empty());

    // Test asset transfer using the Layer2Protocol trait method
    let transfer = AssetTransfer {
        asset_id: asset_id.clone(),
        from: "sender_address".to_string(),
        to: "recipient_address".to_string(),
        amount: 1000,
    };

    let transfer_result = rgb.transfer_asset(transfer).await.unwrap();
    assert_eq!(transfer_result.status, TransactionStatus::Confirmed);

    println!("RGB asset operations completed successfully");
}
/// Test Lightning Network payment functionality
#[tokio::test]
async fn test_lightning_payments() {
    let config = LightningConfig::default();
    let lightning = LightningProtocol::new(config);

    // Initialize and connect
    lightning.initialize().await.unwrap();
    lightning.connect().await.unwrap();

    // Test creating an invoice
    let invoice = lightning
        .create_invoice(1000, "Test payment".to_string(), 3600)
        .await
        .unwrap();
    assert!(!invoice.payment_request.is_empty());

    // Test sending a payment
    let payment = lightning
        .send_payment(invoice.payment_request)
        .await
        .unwrap();
    assert!(!payment.payment_hash.is_empty());

    // Test channel operations
    let peer_pubkey = format!("03{}", "b".repeat(64));
    let channel_id = lightning
        .open_channel(peer_pubkey, 1_000_000)
        .await
        .unwrap();
    assert!(!channel_id.is_empty());

    let channels = lightning.list_channels().await.unwrap();
    assert!(!channels.is_empty());

    println!("Lightning Network payment operations completed successfully");
}

/// Test protocol capabilities
#[tokio::test]
async fn test_protocol_capabilities() {
    // Test Lightning capabilities
    let lightning = LightningProtocol::default();
    lightning.initialize().await.unwrap();

    let lightning_caps = lightning.get_capabilities().await.unwrap();
    assert!(!lightning_caps.supports_assets); // Lightning primarily handles BTC
    assert!(lightning_caps.supports_privacy);

    // Test RGB capabilities
    let rgb = RgbProtocol::default();
    rgb.initialize().await.unwrap();

    let rgb_caps = rgb.get_capabilities().await.unwrap();
    assert!(rgb_caps.supports_assets); // RGB is primarily for assets
    assert!(rgb_caps.supports_smart_contracts);
    assert!(rgb_caps.supports_privacy);

    println!("Protocol capabilities verified successfully");
}

/// Test error handling
#[tokio::test]
async fn test_error_handling() {
    let lightning = LightningProtocol::default();

    // Test operations without connection
    let result = lightning.submit_transaction(b"test_data").await;
    assert!(result.is_err());

    let rgb = RgbProtocol::default();

    // Test operations without connection
    let result = rgb.submit_transaction(b"test_data").await;
    assert!(result.is_err());

    println!("Error handling verified successfully");
}
