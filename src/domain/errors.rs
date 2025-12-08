use axum::{
    http::StatusCode, 
    response::{IntoResponse, Response}, 
    Json
};
use serde_json::json;
use thiserror::Error;
use tracing::error; // <--- ESTA LÍNEA ES LA QUE FALTABA Y CAUSA EL ERROR

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
        // Loguear el error en la terminal
        error!("🔥 ERROR INTERNO: {:?}", self);

        let (status, error_message) = match self {
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::SafetyGuardError => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::ParseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error procesando datos".to_string()),
            AppError::AIError(_) => (StatusCode::BAD_GATEWAY, "Error de comunicación con IA".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}