use axum::{
    Json,
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;
use crate::domain::{models::{User, UserRole}, errors::AppError};
use crate::application::dtos::{CreateUserRequest, UserDto};
use super::admin::AppState;

/// GET /api/admin/users
/// Lista todos los usuarios registrados
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserDto>>, AppError> {
    let users = state.repo.get_all_users().await?;
    
    let dtos = users.into_iter().map(|u| UserDto {
        id: u.id,
        username: u.username,
        role: u.role.to_string(),
    }).collect();

    Ok(Json(dtos))
}

/// POST /api/admin/users
/// Crea un nuevo usuario
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    
    // 1. Verificar si ya existe el usuario
    if let Ok(Some(_)) = state.repo.get_user_by_username(&payload.username).await {
        return Err(AppError::ValidationError("Username already exists".to_string()));
    }

    // 2. Hashear la contraseña
    let hashed_pass = hash(payload.password, DEFAULT_COST)
        .map_err(|e| AppError::ParseError(e.to_string()))?;

    // 3. Crear el modelo de usuario
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: payload.username,
        password_hash: hashed_pass,
        role: payload.role,
    };

    // 4. Guardar en BD
    state.repo.create_user(new_user).await?;

    Ok((StatusCode::CREATED, Json("User created successfully")))
}

/// DELETE /api/admin/users/:username
/// Elimina un usuario por su nombre
pub async fn delete_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    
    // Seguridad básica: No permitir borrar al Admin principal definido en .env
    // para evitar que el sistema se quede sin acceso total accidentalmente.
    let env_admin = std::env::var("ADMIN_USER").unwrap_or("admin".to_string());
    
    if username == env_admin {
        return Err(AppError::SafetyGuardError); // Devuelve 403 Forbidden
    }

    state.repo.delete_user(&username).await?;

    Ok((StatusCode::OK, Json("User deleted")))
}