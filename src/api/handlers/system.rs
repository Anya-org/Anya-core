/// System handlers
use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}

pub async fn system_info() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "name": "Anya Core",
        "version": "1.1.0",
        "status": "operational"
    })))
}
