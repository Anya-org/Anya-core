use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::api::handlers;
#[cfg(feature = "rust-bitcoin")]
use crate::bitcoin::wallet::BitcoinWallet;
use crate::web5::identity::IdentityManager;

use super::handlers::auth::auth_middleware;

/// Configure all API routes
#[cfg(feature = "rust-bitcoin")]
pub fn configure_routes(wallet: Arc<BitcoinWallet>, identity: Arc<IdentityManager>) -> Router {
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(handlers::system::health_check))
        .route("/info", get(handlers::system::system_info))
        .route("/login", post(handlers::auth::login));

    // Bitcoin wallet routes - require authentication
    let wallet_routes = Router::new()
        .route("/wallets", post(handlers::wallet::create_wallet))
        .route("/wallets/:id", get(handlers::wallet::get_wallet))
        .route("/wallets/:id/balance", get(handlers::wallet::get_balance))
        .route(
            "/wallets/:id/address",
            post(handlers::wallet::generate_address),
        )
        .route(
            "/wallets/:id/transactions",
            post(handlers::wallet::send_transaction),
        )
        .route(
            "/wallets/:id/transactions",
            get(handlers::wallet::list_transactions),
        )
        .with_state(wallet.clone());

    // Web5/DID routes - require authentication
    let identity_routes = Router::new()
        .route("/identities", post(handlers::identity::create_identity))
        .route("/identities/:id", get(handlers::identity::get_identity))
        .route("/credentials", post(handlers::identity::create_credential))
        .route("/credentials/:id", get(handlers::identity::get_credential))
        .route(
            "/credentials/verify",
            post(handlers::identity::verify_credential),
        )
        .with_state(identity.clone());

    // DLC routes - require authentication
    let dlc_routes = Router::new()
        .route("/dlc", post(handlers::dlc::create_contract))
        .route("/dlc/:id", get(handlers::dlc::get_contract))
        .route("/dlc/:id/accept", post(handlers::dlc::accept_contract))
        .route("/dlc/:id/finalize", post(handlers::dlc::finalize_contract))
        .route("/dlc/:id/execute", post(handlers::dlc::execute_contract));

    // Combine routes with middleware for authenticated routes
    let authenticated_api = Router::new()
        .merge(wallet_routes)
        .merge(identity_routes)
        .merge(dlc_routes)
        .layer(middleware::from_fn(auth_middleware));

    // Combine public and authenticated routes
    Router::new()
        .nest("/api/v1", public_routes.merge(authenticated_api))
        .layer(TraceLayer::new_for_http())
}

/// Configure all API routes (without Bitcoin functionality)
#[cfg(not(feature = "rust-bitcoin"))]
pub fn configure_routes(identity: Arc<IdentityManager>) -> Router {
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(handlers::system::health_check))
        .route("/info", get(handlers::system::system_info))
        .route("/login", post(handlers::auth::login));

    // Identity routes - require authentication
    let identity_routes = Router::new()
        .route("/identity", post(handlers::identity::create_identity))
        .route("/identity/:id", get(handlers::identity::get_identity))
        .route("/identity/resolve", post(handlers::identity::resolve_identity))
        .route(
            "/credentials/issue",
            post(handlers::identity::issue_credential),
        )
        .route(
            "/credentials/verify",
            post(handlers::identity::verify_credential),
        )
        .with_state(identity.clone());

    // DLC routes - require authentication (without Bitcoin dependency)
    let dlc_routes = Router::new()
        .route("/dlc", post(handlers::dlc::create_contract))
        .route("/dlc/:id", get(handlers::dlc::get_contract))
        .route("/dlc/:id/accept", post(handlers::dlc::accept_contract))
        .route("/dlc/:id/finalize", post(handlers::dlc::finalize_contract))
        .route("/dlc/:id/execute", post(handlers::dlc::execute_contract));

    // Combine routes with middleware for authenticated routes
    let authenticated_api = Router::new()
        .merge(identity_routes)
        .merge(dlc_routes)
        .layer(middleware::from_fn(auth_middleware));

    // Combine public and authenticated routes
    Router::new()
        .nest("/api/v1", public_routes.merge(authenticated_api))
        .layer(TraceLayer::new_for_http())
}
