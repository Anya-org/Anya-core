use anya_core::bitcoin::wallet::{WalletConfig, BitcoinWallet, CoinSelectionStrategy, FeeStrategy, WalletType};
use bitcoin::Network;
use std::path::PathBuf;

use tokio::sync::OnceCell;

async fn create_test_wallet() -> Arc<BitcoinWallet> {
    // Use a static OnceCell to avoid recreating the wallet for every test
    static WALLET: OnceCell<Arc<BitcoinWallet>> = OnceCell::const_new();
    WALLET
        .get_or_init(|| async {
            let config = WalletConfig {
                name: "test-wallet".to_string(),
                database_path: PathBuf::from("/tmp/anya-test-wallet/wallet.db"),
                network: Network::Regtest,
                electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
                password: None,
                mnemonic: None,
                use_taproot: true,
            };
            let wallet = BitcoinWallet::new(config).await.expect("Failed to create wallet");
            Arc::new(wallet)
        })
        .await
        .clone()
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
    api::routes::configure_routes, web5::identity::IdentityManager,
};

/// Test the health check endpoint
#[tokio::test]
async fn test_health_check() {
    let wallet = create_test_wallet().await;
    let identity = Arc::new(IdentityManager::new("test_namespace"));
    let app = configure_routes(wallet, identity);

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
    let identity = Arc::new(IdentityManager::new("test_namespace"));
    let app = configure_routes(wallet, identity);

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
