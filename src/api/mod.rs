pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod server;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Re-export for convenience
pub use error::ApiError;

/// Standard API response format
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub status: StatusCode,
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            message: None,
            status: StatusCode::OK,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            data: Some(data),
            message: Some(message.to_string()),
            status: StatusCode::OK,
        }
    }

    pub fn error(status: StatusCode, message: &str) -> ApiResponse<T> {
        Self {
            data: None,
            message: Some(message.to_string()),
            status,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        let body = match self.data {
            Some(data) => {
                if let Some(message) = self.message {
                    json!({
                        "success": self.status.is_success(),
                        "message": message,
                        "data": data
                    })
                } else {
                    json!({
                        "success": self.status.is_success(),
                        "data": data
                    })
                }
            }
            None => {
                if let Some(message) = self.message {
                    json!({
                        "success": self.status.is_success(),
                        "message": message
                    })
                } else {
                    json!({
                        "success": self.status.is_success()
                    })
                }
            }
        };

        (self.status, Json(body)).into_response()
    }
}
