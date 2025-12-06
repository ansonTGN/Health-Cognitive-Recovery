use axum::{
    response::{Html, IntoResponse, Redirect},
    extract::{State, Form, Query}, // <--- Añadido Query
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

// Estructura para capturar parámetros de URL (ej: /?error=invalid_credentials)
#[derive(Deserialize)]
pub struct LoginParams {
    pub error: Option<String>,
}

/// Página de login (GET "/")
/// Ahora captura parámetros de error para mostrarlos en la UI
pub async fn render_login(
    State(state): State<AppState>,
    Query(params): Query<LoginParams>, // <--- Capturamos query params
) -> impl IntoResponse {
    let mut ctx = Context::new();
    
    // Si hay error en la URL, lo pasamos al template
    if let Some(err) = params.error {
        ctx.insert("error", &err);
    }

    match state.tera.render("login.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            tracing::error!("Error renderizando login: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error interno").into_response()
        },
    }
}

/// Procesa el login de forma segura
pub async fn authenticate(
    State(state): State<AppState>,
    Form(payload): Form<AuthPayload>,
) -> impl IntoResponse {
    
    // 1. Buscar usuario en DB
    let user_result = state.repo.get_user_by_username(&payload.username).await;

    // 2. Lógica de verificación robusta (evita Timing Attacks y Panics)
    let is_valid = match user_result {
        Ok(Some(user)) => {
            // El usuario existe, verificamos hash real
            verify(&payload.password, &user.password_hash).unwrap_or(false)
        },
        Ok(None) => {
            // El usuario NO existe. Simulamos verificación para igualar tiempos.
            // Usamos un hash válido de "dummy"
            let _ = verify(
                "dummy_pass", 
                "$2y$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4h..dummy"
            );
            false
        },
        Err(e) => {
            tracing::error!("Error de base de datos en login: {}", e);
            // En error de DB, redirigimos con mensaje genérico
            return Redirect::to("/?error=system_error").into_response();
        }
    };

    if !is_valid {
        tracing::warn!("Intento de login fallido: {}", payload.username);
        // Redirigimos al login con el error
        return Redirect::to("/?error=invalid_credentials").into_response();
    }

    // 3. Login Exitoso: Recuperamos usuario (sabemos que existe porque is_valid es true)
    // Volvemos a consultar o usamos lógica para extraerlo si lo tuviéramos en el scope anterior.
    // Para simplificar y evitar problemas de borrow checker, lo recuperamos seguro:
    let user = state.repo.get_user_by_username(&payload.username).await.unwrap().unwrap();

    // 4. Construir JWT
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600 * 24; // 24 horas

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
    .unwrap_or_default();

    // 5. Crear Cookie
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

/// Dashboard protegido
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
