use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::domain::models::{AIConfig, UserRole}; 

#[derive(Deserialize, ToSchema)]
pub struct AdminConfigPayload {
    pub config: AIConfig,
    pub force_reset: bool,
}

#[derive(Serialize, ToSchema)]
pub struct IngestionResponse {
    pub id: String,
    pub status: String,
}

// --- NUEVOS DTOs PARA GESTIÓN DE USUARIOS ---

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 chars"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 chars"))]
    pub password: String,
    pub role: UserRole,
}

#[derive(Serialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub role: String,
}