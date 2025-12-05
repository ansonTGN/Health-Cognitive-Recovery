use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("AI Provider error: {0}")]
    AIError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Admin operation requires force flag")]
    SafetyGuardError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::SafetyGuardError => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Error: {}", self)),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}