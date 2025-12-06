use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{ports::{KGRepository, AIService}, errors::AppError};
use crate::application::dtos::AdminConfigPayload;
use tera::Tera;

/// Estado global de la aplicación
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn KGRepository>,
    pub ai_service: Arc<RwLock<dyn AIService>>,
    pub tera: Arc<Tera>,
}

#[utoipa::path(
    post,
    path = "/api/admin/config",
    request_body = AdminConfigPayload,
    responses(
        (status = 200, description = "Config updated"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal error")
    )
)]
pub async fn update_config(
    State(state): State<AppState>,
    Json(payload): Json<AdminConfigPayload>,
) -> Result<impl IntoResponse, AppError> {
    if payload.force_reset {
        // Reset total + recreación de índices + actualización de config IA
        state.repo.reset_database().await?;
        state.repo.create_indexes(payload.config.embedding_dim).await?;
        let mut ai_guard = state.ai_service.write().await;
        ai_guard.update_config(payload.config)?;
        return Ok((StatusCode::OK, Json("System reset and reconfigured successfully")));
    }

    Err(AppError::SafetyGuardError)
}

