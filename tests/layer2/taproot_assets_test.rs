use anya_core::layer2::taproot_assets::{
    create_taproot_asset, create_taproot_asset_mobile, AssetMetadata, Error, IssuanceTx, Network,
};
use std::collections::HashMap;
use tokio;

#[tokio::test]
async fn test_create_taproot_asset() {
    // Create asset metadata for testing
    let metadata = AssetMetadata {
        name: "TestAsset".to_string(),
        supply: 1000000,
        precision: 8,
        issuer: "TestIssuer".to_string(),
        additional_fields: HashMap::new(),
    };

    // Test asset creation
    let result = create_taproot_asset(&metadata, &Network::Testnet).await;
    assert!(result.is_ok(), "Asset creation failed");

    let issuance_tx = result.unwrap();
    assert!(
        !issuance_tx.txid.is_empty(),
        "Transaction ID should not be empty"
    );
    assert!(
        !issuance_tx.asset_id.is_empty(),
        "Asset ID should not be empty"
    );
    assert!(
        !issuance_tx.taproot_script.is_empty(),
        "Taproot script should not be empty"
    );
    assert_eq!(
        issuance_tx.taproot_script, "tr(KEY,{SILENT_LEAF})",
        "Script should match BDF v2.5 format"
    );
}

#[tokio::test]
async fn test_create_taproot_asset_mobile() {
    // Test the mobile-friendly JSON interface
    let metadata_json = r#"{
        "name": "MobileAsset",
        "supply": 2100000,
        "precision": 8,
        "issuer": "MobileIssuer",
        "additional_fields": {}
    }"#;

    let result = create_taproot_asset_mobile(metadata_json, "testnet").await;
    assert!(result.is_ok(), "Mobile asset creation failed");

    let json_result = result.unwrap();
    assert!(
        json_result.contains("txid"),
        "JSON result should contain txid"
    );
    assert!(
        json_result.contains("asset_id"),
        "JSON result should contain asset_id"
    );
    assert!(
        json_result.contains("taproot_script"),
        "JSON result should contain taproot_script"
    );
}
