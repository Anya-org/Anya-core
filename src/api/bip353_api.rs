// [AIR-3][AIS-3][BPC-3][AIT-3] BIP353 API Handler Implementation

use std::sync::{Arc, Mutex};
use axum::{
    extract::{Json, Path, State, Query},
    http::StatusCode,
    response::{Response, IntoResponse},
    routing::{get, post, put},
    Router,
};
use serde::{Serialize, Deserialize};
use tracing::{info, error, warn};

use crate::bip::{
    Bip353, Bip353Config, Bip353Status, Bip353Error, PaymentRecipient,
    BetaAccessManager, BetaAccessConfig, AuthSession, BetaAuthToken, BetaAccessError
};

/// BIP353 API state
pub struct Bip353ApiState {
    pub bip353: Arc<Mutex<Bip353>>,
    pub beta_access: Arc<Mutex<BetaAccessManager>>,
}

/// API response for BIP353 status
#[derive(Debug, Serialize, Deserialize)]
pub struct Bip353StatusResponse {
    pub status: String,
    pub features: Vec<FeatureStatus>,
    pub config: Bip353Config,
}

/// Feature status
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureStatus {
    pub name: String,
    pub enabled: bool,
    pub tier: String,
    pub description: String,
}

/// Request to parse address
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseAddressRequest {
    pub address: String,
}

/// Response for parsed address
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseAddressResponse {
    pub user: String,
    pub domain: String,
    pub full_address: String,
    pub valid: bool,
    pub error: Option<String>,
}

/// Request to resolve address
#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveAddressRequest {
    pub address: String,
}

/// Response for resolved address
#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveAddressResponse {
    pub user: String,
    pub domain: String,
    pub full_address: String,
    pub payment_instruction: Option<String>,
    pub resolved: bool,
    pub error: Option<String>,
}

/// Request to update BIP353 configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBip353ConfigRequest {
    pub status: Bip353Status,
    pub default_resolver: Option<String>,
    pub cache_duration: Option<u64>,
    pub validate_dnssec: Option<bool>,
    pub beta_features: Option<BetaFeaturesRequest>,
}

/// Beta features request
#[derive(Debug, Serialize, Deserialize)]
pub struct BetaFeaturesRequest {
    pub non_ascii_identifiers: Option<bool>,
    pub wildcard_records: Option<bool>,
    pub oob_notifications: Option<bool>,
    pub enhanced_privacy: Option<bool>,
}

/// Response for auth session
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSessionResponse {
    pub session_id: String,
    pub k1: String,
    pub auth_url: Option<String>,
    pub expires_at: u64,
}

/// Request to verify auth
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyAuthRequest {
    pub session_id: String,
    pub signature: String,
    pub pubkey: String,
}

/// Response for auth token
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenResponse {
    pub token: String,
    pub expires_at: u64,
}

/// Beta status response
#[derive(Debug, Serialize, Deserialize)]
pub struct BetaStatusResponse {
    pub enabled: bool,
    pub auth_required: bool,
    pub auth_url: Option<String>,
    pub authorized_pubkeys_count: usize,
}

/// Create BIP353 API routes
pub fn bip353_routes(state: Arc<Bip353ApiState>) -> Router {
    Router::new()
        .route("/api/bip353/status", get(get_status))
        .route("/api/bip353/config", get(get_config))
        .route("/api/bip353/config", put(update_config))
        .route("/api/bip353/parse", post(parse_address))
        .route("/api/bip353/resolve", post(resolve_address))
        .route("/api/bip353/beta/status", get(get_beta_status))
        .route("/api/bip353/beta/auth", post(create_auth_session))
        .route("/api/bip353/beta/verify", post(verify_auth))
        .route("/api/bip353/beta/check", get(check_auth))
        .with_state(state)
}

/// Get BIP353 status
async fn get_status(
    State(state): State<Arc<Bip353ApiState>>,
) -> impl IntoResponse {
    let bip353 = state.bip353.lock().unwrap();
    
    let features = vec![
        FeatureStatus {
            name: "basic_resolution".to_string(),
            enabled: bip353.is_feature_enabled("basic_resolution"),
            tier: "stable".to_string(),
            description: "Basic DNS Payment Instructions resolution".to_string(),
        },
        FeatureStatus {
            name: "dnssec_validation".to_string(),
            enabled: bip353.is_feature_enabled("dnssec_validation"),
            tier: "stable".to_string(),
            description: "DNSSEC validation for secure resolution".to_string(),
        },
        FeatureStatus {
            name: "non_ascii_identifiers".to_string(),
            enabled: bip353.is_feature_enabled("non_ascii_identifiers"),
            tier: "beta".to_string(),
            description: "Support for non-ASCII identifiers (punycode)".to_string(),
        },
        FeatureStatus {
            name: "wildcard_records".to_string(),
            enabled: bip353.is_feature_enabled("wildcard_records"),
            tier: "beta".to_string(),
            description: "Support for wildcard DNS records".to_string(),
        },
        FeatureStatus {
            name: "oob_notifications".to_string(),
            enabled: bip353.is_feature_enabled("oob_notifications"),
            tier: "beta".to_string(),
            description: "Support for out-of-band notifications".to_string(),
        },
        FeatureStatus {
            name: "enhanced_privacy".to_string(),
            enabled: bip353.is_feature_enabled("enhanced_privacy"),
            tier: "beta".to_string(),
            description: "Enhanced privacy routing".to_string(),
        },
    ];
    
    let response = Bip353StatusResponse {
        status: bip353.status().to_string(),
        features,
        config: bip353.config.clone(),
    };
    
    (StatusCode::OK, Json(response))
}

/// Get BIP353 configuration
async fn get_config(
    State(state): State<Arc<Bip353ApiState>>,
) -> impl IntoResponse {
    let bip353 = state.bip353.lock().unwrap();
    
    (StatusCode::OK, Json(bip353.config.clone()))
}

/// Update BIP353 configuration
async fn update_config(
    State(state): State<Arc<Bip353ApiState>>,
    Json(request): Json<UpdateBip353ConfigRequest>,
) -> impl IntoResponse {
    let mut bip353 = state.bip353.lock().unwrap();
    
    // Update config
    let mut config = bip353.config.clone();
    
    config.status = request.status;
    
    if let Some(resolver) = request.default_resolver {
        config.default_resolver = resolver;
    }
    
    if let Some(cache_duration) = request.cache_duration {
        config.cache_duration = cache_duration;
    }
    
    if let Some(validate_dnssec) = request.validate_dnssec {
        config.validate_dnssec = validate_dnssec;
    }
    
    if let Some(beta_features) = request.beta_features {
        if let Some(non_ascii) = beta_features.non_ascii_identifiers {
            config.beta_features.non_ascii_identifiers = non_ascii;
        }
        
        if let Some(wildcard) = beta_features.wildcard_records {
            config.beta_features.wildcard_records = wildcard;
        }
        
        if let Some(oob) = beta_features.oob_notifications {
            config.beta_features.oob_notifications = oob;
        }
        
        if let Some(privacy) = beta_features.enhanced_privacy {
            config.beta_features.enhanced_privacy = privacy;
        }
    }
    
    // Update BIP353 with new config
    match bip353.update_config(config.clone()) {
        Ok(_) => {
            info!("BIP353 configuration updated: {:?}", config);
            (StatusCode::OK, Json(config))
        },
        Err(e) => {
            error!("Failed to update BIP353 configuration: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to update configuration: {}", e)
            })))
        }
    }
}

/// Parse BIP353 address
async fn parse_address(
    State(state): State<Arc<Bip353ApiState>>,
    Json(request): Json<ParseAddressRequest>,
) -> impl IntoResponse {
    let bip353 = state.bip353.lock().unwrap();
    
    match bip353.parse_address(&request.address) {
        Ok(recipient) => {
            (StatusCode::OK, Json(ParseAddressResponse {
                user: recipient.user,
                domain: recipient.domain,
                full_address: recipient.full_address,
                valid: true,
                error: None,
            }))
        },
        Err(e) => {
            (StatusCode::OK, Json(ParseAddressResponse {
                user: "".to_string(),
                domain: "".to_string(),
                full_address: request.address,
                valid: false,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Resolve BIP353 address
async fn resolve_address(
    State(state): State<Arc<Bip353ApiState>>,
    Json(request): Json<ResolveAddressRequest>,
) -> impl IntoResponse {
    let bip353 = state.bip353.lock().unwrap();
    
    // First parse the address
    let mut recipient = match bip353.parse_address(&request.address) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, Json(ResolveAddressResponse {
                user: "".to_string(),
                domain: "".to_string(),
                full_address: request.address,
                payment_instruction: None,
                resolved: false,
                error: Some(format!("Failed to parse address: {}", e)),
            }));
        }
    };
    
    // Now resolve it
    match bip353.resolve(&mut recipient).await {
        Ok(_) => {
            (StatusCode::OK, Json(ResolveAddressResponse {
                user: recipient.user,
                domain: recipient.domain,
                full_address: recipient.full_address,
                payment_instruction: recipient.payment_instruction,
                resolved: true,
                error: None,
            }))
        },
        Err(e) => {
            (StatusCode::OK, Json(ResolveAddressResponse {
                user: recipient.user,
                domain: recipient.domain,
                full_address: recipient.full_address,
                payment_instruction: None,
                resolved: false,
                error: Some(format!("Failed to resolve: {}", e)),
            }))
        }
    }
}

/// Get beta access status
async fn get_beta_status(
    State(state): State<Arc<Bip353ApiState>>,
) -> impl IntoResponse {
    let beta_access = state.beta_access.lock().unwrap();
    let config = beta_access.get_config();
    
    let response = BetaStatusResponse {
        enabled: config.enabled,
        auth_required: config.enabled,
        auth_url: config.auth_url.clone(),
        authorized_pubkeys_count: config.authorized_pubkeys.len(),
    };
    
    (StatusCode::OK, Json(response))
}

/// Create LNURL auth session for beta access
async fn create_auth_session(
    State(state): State<Arc<Bip353ApiState>>,
) -> impl IntoResponse {
    let beta_access = state.beta_access.lock().unwrap();
    
    if !beta_access.get_config().enabled {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Beta access auth not enabled"
        })));
    }
    
    let session = beta_access.create_session();
    let auth_url = beta_access.get_auth_url(&session);
    
    let response = AuthSessionResponse {
        session_id: session.id,
        k1: session.k1,
        auth_url,
        expires_at: session.expires_at,
    };
    
    (StatusCode::OK, Json(response))
}

/// Verify LNURL auth for beta access
async fn verify_auth(
    State(state): State<Arc<Bip353ApiState>>,
    Json(request): Json<VerifyAuthRequest>,
) -> impl IntoResponse {
    let beta_access = state.beta_access.lock().unwrap();
    
    match beta_access.verify_auth(&request.session_id, &request.signature, &request.pubkey) {
        Ok(session) => {
            // Create auth token
            let token = BetaAuthToken::new(session.id, session.expires_at);
            
            let response = AuthTokenResponse {
                token: token.encode(),
                expires_at: token.expires_at,
            };
            
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let status = match e {
                BetaAccessError::InvalidSession(_) => StatusCode::BAD_REQUEST,
                BetaAccessError::ExpiredSession => StatusCode::UNAUTHORIZED,
                BetaAccessError::NotAuthorized => StatusCode::FORBIDDEN,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            
            (status, Json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

/// Check auth token for beta access
async fn check_auth(
    State(state): State<Arc<Bip353ApiState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(t) => t,
        None => {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Missing token parameter"
            })));
        }
    };
    
    // Decode token
    let token = match BetaAuthToken::decode(token) {
        Ok(t) => t,
        Err(e) => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": format!("Invalid token: {}", e)
            })));
        }
    };
    
    // Check if session is valid
    let beta_access = state.beta_access.lock().unwrap();
    if !beta_access.is_authenticated(&token.session_id) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid or expired session"
        })));
    }
    
    (StatusCode::OK, Json(serde_json::json!({
        "authenticated": true,
        "expires_at": token.expires_at
    })))
} 