use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use axum_extra::extract::cookie::{SignedCookieJar, Key};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::domain::models::{Claims, UserRole};

// Clave estática para firma de cookies (Debe coincidir con main)
pub static COOKIE_KEY: &[u8] = b"SUPER_SECURE_KEY_THAT_MUST_BE_VERY_LONG_IN_PROD_123456";

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    
    // Extraer cookies firmadas manualmente
    let headers = req.headers().clone();
    let key = Key::from(COOKIE_KEY);
    // En axum-extra 0.9, from_headers toma (headers, key) para SignedCookieJar
    let jar = SignedCookieJar::from_headers(&headers, key);
    
    let token_cookie = jar.get("lamuralla_jwt").map(|c| c.value().to_string());

    let token = match token_cookie {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "fallback_secret_dev_only".to_string());
    
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

pub async fn require_admin(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<Claims>().ok_or(StatusCode::UNAUTHORIZED)?;
    
    if claims.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}