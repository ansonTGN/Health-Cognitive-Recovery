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
use secrecy::SecretString;
use tera::Tera;
use bcrypt::{hash, DEFAULT_COST};

use crate::domain::models::*;
use crate::domain::ports::KGRepository;
use crate::infrastructure::ai::rig_client::RigAIService;
use crate::infrastructure::persistence::neo4j_repo::Neo4jRepo;
use crate::interface::handlers::{
    admin::{self, AppState},
    ingest,
    graph,
    ui,
    chat,
    reasoning,
    export,
    users, // <--- IMPORTADO
};

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
        AIConfig,
        AIProvider,
        IngestionRequest,
        VisNode,
        VisEdge,
        GraphDataResponse,
        ChatRequest,
        ChatResponse,
        InferredRelation,
        ExportParams,
        ExportFormat,
        // (Opcional) Agrega CreateUserRequest y UserDto aquí si quieres documentarlos
    )),
    tags((name = "lamuralla", description = "Mental Health API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 Starting La Muralla Security Core...");

    // 1. Config IA
    let provider_str = std::env::var("AI_PROVIDER").unwrap_or_else(|_| "openai".to_string());
    let api_key_str = std::env::var("AI_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .unwrap_or_default();
    let model_name = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4o".to_string());
    let embedding_model =
        std::env::var("AI_EMBEDDING_MODEL").unwrap_or_else(|_| "text-embedding-3-small".to_string());
    let embedding_dim = std::env::var("AI_EMBEDDING_DIM")
        .unwrap_or("1536".to_string())
        .parse::<usize>()?;
    let base_url = std::env::var("AI_BASE_URL").ok();

    let provider = match provider_str.to_lowercase().as_str() {
        "ollama" => AIProvider::Ollama,
        "groq" => AIProvider::Groq,
        _ => AIProvider::OpenAI,
    };

    let initial_config = AIConfig {
        provider,
        model_name,
        embedding_model,
        api_key: SecretString::new(api_key_str.into()),
        embedding_dim,
        base_url,
    };

    // 2. Neo4j
    let uri = std::env::var("NEO4J_URI").expect("NEO4J_URI required");
    let user = std::env::var("NEO4J_USER").expect("NEO4J_USER required");
    let pass = std::env::var("NEO4J_PASS").expect("NEO4J_PASS required");

    let graph = Arc::new(Graph::new(&uri, &user, &pass).await?);
    let repo = Arc::new(Neo4jRepo::new(graph.clone()));

    let _ = repo.create_indexes(embedding_dim).await;

    // 3. Usuario admin (Verifica/Crea el admin del .env)
    let admin_user = std::env::var("ADMIN_USER").unwrap_or("admin".to_string());
    let admin_pass = std::env::var("ADMIN_PASS").unwrap_or("admin123".to_string());
    let hashed_pass = hash(admin_pass, DEFAULT_COST)?;
    repo.ensure_admin_exists(&admin_user, &hashed_pass).await?;

    // 4. Servicios y estado
    let ai_service = Arc::new(RwLock::new(RigAIService::new(initial_config)));
    let tera = Tera::new("templates/**/*.html")?;

    let app_state = AppState {
        repo: repo.clone(),
        ai_service,
        tera: Arc::new(tera),
    };

    // 5. Rutas públicas
    let public_routes = Router::new()
        .route("/", get(ui::render_login).post(ui::authenticate))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .with_state(app_state.clone()); 

    // 6. Rutas de usuario autenticado
    let user_routes = Router::new()
        .route("/dashboard", get(ui::render_dashboard_guarded))
        .route("/api/graph", get(graph::get_graph))
        .route("/api/graph/concept/{name}", get(graph::get_concept_neighborhood))
        .route("/api/chat", post(chat::chat_handler))
        .route("/api/export", get(export::export_knowledge_graph))
        .route("/api/reasoning/run", post(reasoning::run_reasoning))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::interface::middleware::auth_middleware,
        ))
        .with_state(app_state.clone());

    // 7. Rutas admin (ingest + config + USERS)
    let admin_routes = Router::new()
        .route("/api/admin/config", post(admin::update_config))
        .route("/api/ingest", post(ingest::ingest_document))
        // NUEVAS RUTAS 👇
        .route("/api/admin/users", get(users::list_users).post(users::create_user))
        .route("/api/admin/users/:username", axum::routing::delete(users::delete_user))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::interface::middleware::require_admin,
        ))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::interface::middleware::auth_middleware,
        ))
        .with_state(app_state.clone());

    // 8. Router principal
    let app = Router::new()
        .merge(public_routes)
        .merge(user_routes)
        .merge(admin_routes)
        .route("/logout", get(|| async { Redirect::to("/") }))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // 9. Lanzar servidor
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("✅ Secure Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}


