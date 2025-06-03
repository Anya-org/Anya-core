use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

use crate::bitcoin::error::BitcoinError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication required: {0}")]
    AuthenticationRequired(String),

    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Bitcoin operation failed: {0}")]
    BitcoinError(#[from] BitcoinError),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::AuthenticationRequired(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::AuthorizationFailed(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::BitcoinError(e) => match e {
                BitcoinError::WalletNotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
                BitcoinError::InvalidAddress(_) => (StatusCode::BAD_REQUEST, e.to_string()),
                BitcoinError::InsufficientFunds(_) => (StatusCode::BAD_REQUEST, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            },
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "message": error_message
        }));

        (status, body).into_response()
    }
}
