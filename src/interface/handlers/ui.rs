use axum::{
    response::{Html, IntoResponse, Redirect},
    extract::{State, Form},
    http::StatusCode,
    Extension,
};
use axum_extra::extract::cookie::{Cookie, SameSite, SignedCookieJar}; 
// use std::sync::Arc; <-- ELIMINADO
use tera::{Context, Tera};
use serde::Deserialize;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::interface::handlers::admin::AppState;
use crate::domain::models::Claims;

#[derive(Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String,
}

pub async fn render_login() -> impl IntoResponse {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => return Html(format!("<h1>Templates Error: {}</h1>", e)).into_response(),
    };
    match tera.render("login.html", &Context::new()) {
        Ok(html) => Html(html).into_response(),
        Err(err) => Html(format!("<h1>Render Error</h1><p>{}</p>", err)).into_response(),
    }
}

pub async fn authenticate(
    State(state): State<AppState>, 
    jar: SignedCookieJar, 
    Form(payload): Form<AuthPayload>,
) -> impl IntoResponse {
    
    let user_opt = state.repo.get_user_by_username(&payload.username).await.unwrap_or(None);

    let is_valid = if let Some(ref user) = user_opt {
        verify(&payload.password, &user.password_hash).unwrap_or(false)
    } else {
        false
    };

    if is_valid {
        let user = user_opt.unwrap();
        
        let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + 3600; 
        let claims = Claims {
            sub: user.username,
            role: user.role,
            exp: expiration,
        };
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "fallback_secret_dev_only".to_string());
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();

        let mut cookie = Cookie::new("lamuralla_jwt", token);
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(SameSite::Strict);
        cookie.set_path("/");

        let updated_jar = jar.add(cookie);

        (updated_jar, Redirect::to("/dashboard")).into_response()
    } else {
        let mut ctx = Context::new();
        ctx.insert("error", &true);
        let tera = Tera::new("templates/**/*.html").unwrap();
        let html = tera.render("login.html", &ctx).unwrap_or_default();
        (StatusCode::UNAUTHORIZED, Html(html)).into_response()
    }
}

pub async fn render_dashboard_guarded(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let mut ctx = Context::new();
    
    let ai_guard = state.ai_service.read().await;
    let config = ai_guard.get_config();

    ctx.insert("config", &serde_json::json!({
        "model_name": config.model_name,
        "embedding_dim": config.embedding_dim
    }));

    ctx.insert("username", &claims.sub);
    ctx.insert("role", &claims.role.to_string());

    match state.tera.render("dashboard.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(err) => Html(format!("<h1>Error rendering template</h1><p>{}</p>", err)).into_response(),
    }
}