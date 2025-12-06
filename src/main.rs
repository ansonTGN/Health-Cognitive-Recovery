mod domain;
mod application;
mod infrastructure;
mod interface;

use axum::{
    routing::{post, get}, 
    Router, 
    middleware,
    response::Redirect, 
    http::{HeaderValue, header},
    Extension, // Importante para inyectar la Key globalmente
}; 
use std::sync::Arc;
use tokio::sync::RwLock;
use neo4rs::Graph;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use secrecy::SecretString;
use tera::Tera;
use bcrypt::{hash, DEFAULT_COST};
use axum_extra::extract::cookie::Key;

use crate::domain::models::*;
use crate::domain::ports::KGRepository; 
use crate::infrastructure::ai::rig_client::RigAIService;
use crate::infrastructure::persistence::neo4j_repo::Neo4jRepo;
// Importamos AppState desde admin
use crate::interface::handlers::{admin::{self, AppState}, ingest, graph, ui, chat, reasoning, export}; 
use crate::interface::middleware::COOKIE_KEY;

#[derive(OpenApi)]
#[openapi(
    paths(
        interface::handlers::admin::update_config,
        interface::handlers::ingest::ingest_document,
        interface::handlers::graph::get_graph,
        interface::handlers::graph::get_concept_neighborhood,
        interface::handlers::chat::chat_handler,
        interface::handlers::reasoning::run_reasoning,
        interface::handlers::export::export_knowledge_graph
    ),
    components(schemas(
        AIConfig, AIProvider, IngestionRequest, VisNode, VisEdge, 
        GraphDataResponse, ChatRequest, ChatResponse, InferredRelation, 
        ExportParams, ExportFormat
    )),
    tags((name = "lamuralla", description = "Mental Health API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 Starting La Muralla Security Core...");

    // 1. Config AI
    let provider_str = std::env::var("AI_PROVIDER").unwrap_or_else(|_| "openai".to_string());
    let api_key_str = std::env::var("AI_API_KEY").or_else(|_| std::env::var("OPENAI_API_KEY")).unwrap_or_default();
    let model_name = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4o".to_string());
    let embedding_model = std::env::var("AI_EMBEDDING_MODEL").unwrap_or_else(|_| "text-embedding-3-small".to_string());
    let embedding_dim = std::env::var("AI_EMBEDDING_DIM").unwrap_or("1536".to_string()).parse::<usize>()?;
    let base_url = std::env::var("AI_BASE_URL").ok();

    let provider = match provider_str.to_lowercase().as_str() {
        "ollama" => AIProvider::Ollama,
        "groq" => AIProvider::Groq,
        _ => AIProvider::OpenAI,
    };

    let initial_config = AIConfig {
        provider, model_name, embedding_model, 
        api_key: SecretString::new(api_key_str.into()), 
        embedding_dim, base_url,
    };

    // 2. DB Setup
    let uri = std::env::var("NEO4J_URI").expect("NEO4J_URI required");
    let user = std::env::var("NEO4J_USER").expect("NEO4J_USER required");
    let pass = std::env::var("NEO4J_PASS").expect("NEO4J_PASS required");
    
    let graph = Arc::new(Graph::new(&uri, &user, &pass).await?);
    let repo = Arc::new(Neo4jRepo::new(graph.clone()));
    
    // Crear índices
    let _ = repo.create_indexes(embedding_dim).await;
    
    // Crear usuario admin por defecto
    let admin_user = std::env::var("ADMIN_USER").unwrap_or("admin".to_string());
    let admin_pass = std::env::var("ADMIN_PASS").unwrap_or("admin123".to_string());
    let hashed_pass = hash(admin_pass, DEFAULT_COST)?;
    repo.ensure_admin_exists(&admin_user, &hashed_pass).await?;

    // 3. State & Templating
    let ai_service = Arc::new(RwLock::new(RigAIService::new(initial_config)));
    let tera = Tera::new("templates/**/*.html")?;
    
    // CORRECCIÓN: AppState sin campo 'key', usando Arc internos.
    let app_state = AppState { 
        repo: repo.clone(), 
        ai_service, 
        tera: Arc::new(tera), 
    };

    // 4. Security Layers
    let governor_conf = Arc::new(GovernorConfigBuilder::default()
        .per_second(5) 
        .burst_size(10)
        .finish()
        .unwrap());

    // 5. Routing
    let public_routes = Router::new()
        .route("/", get(ui::render_login).post(ui::authenticate))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let user_routes = Router::new()
        .route("/dashboard", get(ui::render_dashboard_guarded))
        .route("/api/graph", get(graph::get_graph))
        .route("/api/graph/concept/{name}", get(graph::get_concept_neighborhood))
        .route("/api/chat", post(chat::chat_handler))
        .route("/api/export", get(export::export_knowledge_graph))
        .route("/api/reasoning/run", post(reasoning::run_reasoning))
        .route_layer(middleware::from_fn(crate::interface::middleware::auth_middleware));

    let admin_routes = Router::new()
        .route("/api/admin/config", post(admin::update_config))
        .route("/api/ingest", post(ingest::ingest_document))
        .route_layer(middleware::from_fn(crate::interface::middleware::require_admin))
        .route_layer(middleware::from_fn(crate::interface::middleware::auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(user_routes)
        .merge(admin_routes)
        .route("/logout", get(|| async { Redirect::to("/") }))
        // IMPORTANTE: Aquí inyectamos la Key para las Cookies firmadas
        .layer(Extension(Key::from(COOKIE_KEY))) 
        .layer(GovernorLayer { config: governor_conf })
        .layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("✅ Secure Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}