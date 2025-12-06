use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use axum_extra::extract::cookie::{CookieJar, Key};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::domain::models::{Claims, UserRole};

// En producción, cargar esto de ENV o usar Key::generate() una vez al inicio
pub static COOKIE_KEY: &[u8] = b"SUPER_SECURE_KEY_THAT_MUST_BE_VERY_LONG_IN_PROD_123456";

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Extraer cookie
    let headers = req.headers().clone(); // Clonar para CookieJar
    let jar = CookieJar::from_headers(&headers, Key::from(COOKIE_KEY));
    
    let token_cookie = jar.get("lamuralla_jwt").map(|c| c.value().to_string());

    let token = match token_cookie {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // 2. Validar JWT
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "fallback_secret_dev_only".to_string());
    
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 3. Inyectar usuario (Claims) en la request para que los handlers lo usen
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

// Middleware de Autorización RBAC
pub async fn require_admin(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Se asume que auth_middleware corrió antes y puso los claims
    let claims = req.extensions().get::<Claims>().ok_or(StatusCode::UNAUTHORIZED)?;
    
    if claims.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}