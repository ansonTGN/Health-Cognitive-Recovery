use axum::{
    Json, 
    extract::{State, FromRef}, 
    http::StatusCode, 
    response::IntoResponse
};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{ports::{KGRepository, AIService}, errors::AppError};
use crate::application::dtos::AdminConfigPayload;
use tera::Tera;
use axum_extra::extract::cookie::Key; 

// ESTRUCTURA REFACTORIZADA:
// 1. Deriva Clone (porque todos sus campos son Arc o clonables baratos)
// 2. Contiene los Arc internamente.
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn KGRepository>,
    pub ai_service: Arc<RwLock<dyn AIService>>, 
    pub tera: Arc<Tera>, // Tera envuelto en Arc para clonación barata
    pub key: Key,        // Key para firmar cookies
}

// Implementación necesaria para que SignedCookieJar funcione
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
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
    State(state): State<AppState>, // <-- Sin Arc<>
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

    Err(AppError::SafetyGuardError)
}