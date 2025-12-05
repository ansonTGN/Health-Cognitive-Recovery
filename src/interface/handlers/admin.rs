use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{ports::{KGRepository, AIService}, errors::AppError};
use crate::application::dtos::AdminConfigPayload;
use tera::Tera;

// Estado compartido (ver main.rs)
pub struct AppState {
    pub repo: Arc<dyn KGRepository>,
    pub ai_service: Arc<RwLock<dyn AIService>>, // RwLock para poder actualizar config
    pub tera: Tera, // <-- NUEVO CAMPO
}

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
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AdminConfigPayload>,
) -> Result<impl IntoResponse, AppError> {
    
    if payload.force_reset {
        // 1. Limpiar BD
        state.repo.reset_database().await?;
        
        // 2. Recrear índices según nueva dimensión
        state.repo.create_indexes(payload.config.embedding_dim).await?;
        
        // 3. Actualizar Servicio de IA
        let mut ai_guard = state.ai_service.write().await;
        ai_guard.update_config(payload.config)?;
        
        return Ok((StatusCode::OK, Json("System reset and reconfigured successfully")));
    }

    // Si intenta cambiar configuración sin force_reset, denegar si implica cambio estructural
    // Por simplicidad, exigimos force_reset para cualquier cambio de configuración en este endpoint crítico
    Err(AppError::SafetyGuardError)
}