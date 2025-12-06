use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::domain::models::{Claims, UserRole};

/// Middleware de autenticación basado en cookie "lamuralla_jwt"
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Leer cabecera Cookie
    let cookie_header = req
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Buscar cookie "lamuralla_jwt"
    let token_opt = cookie_header
        .split(';')
        .map(|s| s.trim())
        .find_map(|part| {
            if let Some(rest) = part.strip_prefix("lamuralla_jwt=") {
                Some(rest.to_string())
            } else {
                None
            }
        });

    let token = match token_opt {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validar JWT
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "fallback_secret_dev_only".to_string());

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Adjuntar claims al Request para su uso posterior
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

/// Middleware adicional para restringir a rol Admin
pub async fn require_admin(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claims.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}
