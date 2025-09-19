use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use super::handlers::auth::auth_middleware;
use crate::api::handlers;
#[cfg(feature = "dwn")]
use crate::handlers::dwn;
#[cfg(feature = "bitcoin")]
use crate::handlers::rgb;
use crate::web::web5_adapter::Web5Adapter;

#[cfg(feature = "bitcoin")]
use crate::bitcoin::wallet::Wallet;

#[cfg(not(feature = "bitcoin"))]
pub struct Wallet;

/// Configure all API routes
pub fn configure_routes(
    #[cfg(feature = "bitcoin")] wallet: Arc<Wallet>,
    web5_adapter: Arc<Web5Adapter>,
) -> Router {
    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(handlers::system::health_check))
        .route("/info", get(handlers::system::system_info))
        .route("/login", post(handlers::auth::login));

    // Identity routes - require authentication
    let identity_routes = Router::new()
        .route("/identity", post(handlers::identity::create_identity))
        .route("/identity/{id}", get(handlers::identity::get_identity))
        .route(
            "/identity/resolve",
            post(handlers::identity::resolve_identity),
        )
        .route(
            "/credentials/issue",
            post(handlers::identity::issue_credential),
        )
        .route(
            "/credentials/verify",
            post(handlers::identity::verify_credential),
        )
        .with_state(web5_adapter.clone());

    // Bitcoin routes (conditional compilation)
    #[cfg(feature = "bitcoin")]
    let bitcoin_routes = {
        // Bitcoin wallet routes - require authentication
        Router::new()
            .route("/wallet/balance", get(handlers::wallet::get_balance))
            .route("/wallet/address", get(handlers::wallet::get_new_address))
            .route("/wallet/send", post(handlers::wallet::send_transaction))
            .route(
                "/wallet/history",
                get(handlers::wallet::get_transaction_history),
            )
            .route("/wallet/backup", post(handlers::wallet::backup_wallet))
            .route("/wallet/restore", post(handlers::wallet::restore_wallet))
            .with_state(wallet)
    };

    #[cfg(not(feature = "bitcoin"))]
    let bitcoin_routes = Router::new(); // Empty router when bitcoin feature is disabled

    // DWN (Decentralized Web Node) routes - require authentication
    #[cfg(feature = "dwn")]
    let dwn_routes = Router::new()
        .route("/dwn/protocols", get(dwn::list_protocols))
        .route("/dwn/protocols", post(dwn::create_protocol))
        .route("/dwn/records", get(dwn::query_records))
        .route("/dwn/records", post(dwn::create_record))
        .route("/dwn/records/{id}", get(dwn::get_record)); // update/delete stubs removed pending handler impl
    #[cfg(not(feature = "dwn"))]
    let dwn_routes = Router::new();

    // RGB routes - require authentication (conditional)
    #[cfg(feature = "bitcoin")]
    let rgb_routes = Router::new()
        .route("/rgb/assets", get(rgb::list_assets))
        .route("/rgb/assets", post(rgb::create_asset))
        .route("/rgb/assets/{id}", get(rgb::get_asset))
        .route("/rgb/assets/{id}/transfer", post(rgb::transfer_asset))
        .route("/rgb/assets/{id}/history", get(rgb::get_asset_history));

    #[cfg(not(feature = "bitcoin"))]
    let rgb_routes = Router::new(); // Empty router when bitcoin feature is disabled

    // DLC routes - require authentication
    let dlc_routes = Router::new()
        .route("/dlc", post(handlers::dlc::create_contract))
        .route("/dlc/{id}", get(handlers::dlc::get_contract))
        .route("/dlc/{id}/accept", post(handlers::dlc::accept_contract))
        .route("/dlc/{id}/finalize", post(handlers::dlc::finalize_contract))
        .route("/dlc/{id}/execute", post(handlers::dlc::execute_contract));

    // Authenticated routes
    let authenticated_api = identity_routes
        .merge(bitcoin_routes)
        .merge(dwn_routes)
        .merge(rgb_routes)
        .merge(dlc_routes)
        .layer(middleware::from_fn(auth_middleware));

    // Combine public and authenticated routes
    Router::new()
        .nest("/api/v1", public_routes.merge(authenticated_api))
        .layer(TraceLayer::new_for_http())
}
