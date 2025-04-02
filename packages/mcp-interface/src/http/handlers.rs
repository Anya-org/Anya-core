// HTTP Handlers [AIR-3][BPC-3]
//
// This module defines the HTTP route handlers for the MCP interface
// following the Bitcoin Development Framework v2.5 requirements

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::{Value, json};
use log::debug;
use chrono::Utc;

use crate::compliance;
use super::transport::HttpServerState;

/// Root handler
pub(crate) async fn root_handler() -> &'static str {
    "MCP HTTP Interface"
}

/// Health check handler [AIM-3]
pub(crate) async fn health_handler(State(state): State<HttpServerState>) -> (StatusCode, Json<Value>) {
    let status = state.get_health_status();
    let response = json!({
        "status": status,
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": state.metrics.get_uptime_seconds(),
        "requests": state.metrics.get_total_requests(),
    });
    
    (StatusCode::OK, Json(response))
}

/// BIP status handler [BPC-3]
pub(crate) async fn bip_status_handler() -> Json<Value> {
    let bips = compliance::get_supported_bips();
    let bip_data = bips.iter().map(|bip| {
        json!({
            "number": bip.number,
            "name": bip.name,
            "supported": bip.supported,
            "compliant": bip.compliant
        })
    }).collect::<Vec<_>>();
    
    Json(json!({
        "bips": bip_data
    }))
}

/// API handler
pub(crate) async fn api_handler(
    Path(version): Path<String>,
    Path(endpoint): Path<String>,
    State(state): State<HttpServerState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    // Increment request counter
    state.metrics.increment_request_count();
    
    debug!(
        "API request: version={}, endpoint={}, payload={:?}",
        version, endpoint, payload
    );
    
    // Process the request based on version and endpoint
    let response = match (version.as_str(), endpoint.as_str()) {
        ("v1", "ping") => {
            json!({
                "result": "pong",
                "timestamp": Utc::now().to_rfc3339()
            })
        }
        _ => {
            json!({
                "error": format!("Unsupported API version or endpoint: {}/{}", version, endpoint)
            })
        }
    };
    
    Json(response)
}

/// Metrics handler [AIM-3]
pub(crate) async fn metrics_handler(State(state): State<HttpServerState>) -> Json<Value> {
    let metrics = state.metrics.get_metrics();
    
    Json(metrics.as_json())
}
