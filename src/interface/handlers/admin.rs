use axum::{
    Json, 
    extract::State, 
    http::StatusCode, 
    response::IntoResponse
};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{ports::{KGRepository, AIService}, errors::AppError};
use crate::application::dtos::AdminConfigPayload;
use tera::Tera;
// Borramos el import de Key

// AppState limpio, solo dependencias de negocio
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn KGRepository>,
    pub ai_service: Arc<RwLock<dyn AIService>>, 
    pub tera: Arc<Tera>,
    // Eliminamos: pub key: Key
}

// Eliminamos: impl FromRef<AppState> for Key...

#[utoipa::path(
    post,
    path = "/api/admin/config",
    request_body = AdminConfigPayload,
    responses(
        (status = 200, description = "Configuration updated successfully"),
        (status = 403, description = "Force reset required for model change"),
        (status = 500, description = "Internal error")
    )
)]
pub async fn update_config(
    State(state): State<AppState>,
    Json(payload): Json<AdminConfigPayload>,
) -> Result<impl IntoResponse, AppError> {
    
    if payload.force_reset {
        state.repo.reset_database().await?;
        state.repo.create_indexes(payload.config.embedding_dim).await?;
        
        let mut ai_guard = state.ai_service.write().await;
        ai_guard.update_config(payload.config)?;
        
        return Ok((StatusCode::OK, Json("System reset and reconfigured successfully")));
    }

    Err(AppError::SafetyGuardError)
}