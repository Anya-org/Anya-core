/// Wallet handlers
use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

pub async fn create_wallet() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "success": true,
        "message": "Wallet created"
    })))
}

pub async fn get_wallet() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "wallet_id": "example",
        "balance": 0
    })))
}

pub async fn get_balance() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "balance": 0,
        "currency": "BTC"
    })))
}

pub async fn generate_address() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "address": "bc1qexample"
    })))
}

pub async fn send_transaction() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "txid": "example_txid"
    })))
}

pub async fn list_transactions() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "transactions": []
    })))
}
