use axum::{
    response::{Html, IntoResponse, Redirect},
    extract::{State, Form},
    http::{StatusCode, header},
    Extension,
};
use tera::Context;
use serde::Deserialize;
use bcrypt::verify;
use jsonwebtoken::{encode, Header as JwtHeader, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::interface::handlers::admin::AppState;
use crate::domain::models::Claims;

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

/// Página de login (GET "/")
pub async fn render_login(State(state): State<AppState>) -> impl IntoResponse {
    match state.tera.render("login.html", &Context::new()) {
        Ok(html) => Html(html).into_response(),
        Err(err) => Html(format!("<h1>Render Error</h1><p>{}</p>", err)).into_response(),
    }
}

/// Procesa el login, emite JWT en cookie y redirige al dashboard
pub async fn authenticate(
    State(state): State<AppState>,
    Form(payload): Form<AuthPayload>,
) -> impl IntoResponse {
    // Buscar usuario
    let user_opt = match state.repo.get_user_by_username(&payload.username).await {
        Ok(u) => u,
        Err(_) => None,
    };

    let is_valid = if let Some(ref user) = user_opt {
        verify(&payload.password, &user.password_hash).unwrap_or(false)
    } else {
        false
    };

    if !is_valid {
        // Credenciales incorrectas
        return (
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        )
            .into_response();
    }

    let user = user_opt.unwrap();

    // Construir claims JWT
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600; // 1h

    let claims = Claims {
        sub: user.username,
        role: user.role,
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "fallback_secret_dev_only".to_string());

    let token = encode(
        &JwtHeader::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    // Creamos cookie manualmente (el JWT ya va firmado)
    let cookie_value = format!(
        "lamuralla_jwt={}; HttpOnly; SameSite=Strict; Path=/",
        token
    );

    let mut response = Redirect::to("/dashboard").into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, header::HeaderValue::from_str(&cookie_value).unwrap());

    response
}

/// Dashboard protegido, solo accesible si el middleware ha adjuntado Claims
pub async fn render_dashboard_guarded(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut ctx = Context::new();

    let ai_guard = state.ai_service.read().await;
    let config = ai_guard.get_config();

    ctx.insert(
        "config",
        &serde_json::json!({
            "model_name": config.model_name,
            "embedding_dim": config.embedding_dim
        }),
    );

    ctx.insert("username", &claims.sub);
    ctx.insert("role", &claims.role.to_string());

    match state.tera.render("dashboard.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(err) => Html(format!("<h1>Error rendering template</h1><p>{}</p>", err)).into_response(),
    }
}
