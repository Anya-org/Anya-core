//! Tests for the documentation
//!
//! This file contains tests that verify the correctness of the code examples in the
//! documentation. It also includes integration tests that check the API endpoints.

use anya_core::AnyaConfig;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

use anya_core::{
    api::routes::configure_routes, bitcoin::wallet::BitcoinWallet, web5::identity::IdentityManager,
};

/// Test the health check endpoint
#[tokio::test]
async fn test_health_check() {
    let wallet = Arc::new(BitcoinWallet::new_for_test("test_wallet").unwrap());
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
    let wallet = Arc::new(BitcoinWallet::new_for_test("test_wallet").unwrap());
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
