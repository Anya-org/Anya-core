use anya_core::bitcoin::wallet::{WalletConfig, CoinSelectionStrategy, FeeStrategy, WalletType, Wallet};
use bitcoin::Network;
use std::path::PathBuf;

use tokio::sync::OnceCell;

async fn create_test_wallet() -> Arc<Wallet> {
    let config = WalletConfig {
        name: "test-wallet".to_string(),
        network: Network::Regtest,
        wallet_type: WalletType::Taproot,
        seed_phrase: None,
        password: None,
        receive_descriptor: "tr([73c5da0a/86'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/0/*)".to_string(),
        change_descriptor: "tr([73c5da0a/86'/1'/0']xprv9xgqHN7yz9MwCkxsBPN5qetuNdQSUttZNKw1dcYTV4mTp8ZrKLRPXBThPxq9h3wcAAJVH5qQCk99URy2CQHEMnMKUNpUorQJpXbgJC6C1HR/1/*)".to_string(),
        xpub: None,
        data_dir: PathBuf::from("/tmp/anya-test-wallet"),
        use_rpc: false,
        coin_selection: CoinSelectionStrategy::BranchAndBound,
        gap_limit: 20,
        min_confirmations: 1,
        fee_strategy: FeeStrategy::Medium,
    };
    let wallet = Wallet::new(config, None);
    Arc::new(wallet)
}
// Tests for the documentation
//
// This file contains tests that verify the correctness of the code examples in the
// documentation. It also includes integration tests that check the API endpoints.

use anya_core::AnyaConfig;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

use anya_core::{
    api::routes::configure_routes,
};
use anya_core::web::web5_adapter::Web5Adapter;

/// Test the health check endpoint
#[tokio::test]
async fn test_health_check() {
    let wallet = create_test_wallet().await;
    let web5_adapter = Arc::new(Web5Adapter::new("http://localhost:8080"));
    let app = configure_routes(wallet, web5_adapter);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

/// Test the system information endpoint
#[tokio::test]
async fn test_system_info() {
    let wallet = create_test_wallet().await;
    let web5_adapter = Arc::new(Web5Adapter::new("http://localhost:8080"));
    let app = configure_routes(wallet, web5_adapter);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
