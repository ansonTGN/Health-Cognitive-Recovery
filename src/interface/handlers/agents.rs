use axum::{Json, extract::{State, Extension}};
use crate::application::agent_service::AgentService;
use crate::domain::models::{AgentChatRequest, AgentChatResponse, AgentConfig, ToolDefinition, Claims}; 
use crate::domain::errors::AppError;
use super::admin::AppState;
use tracing::info; // Importamos el macro de log

#[utoipa::path(
    get,
    path = "/api/agents",
    responses((status = 200, body = Vec<AgentConfig>))
)]
pub async fn list_agents(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>, // Capturamos quién hace la petición
) -> Result<Json<Vec<AgentConfig>>, AppError> {
    
    info!("📋 ACCESO: Usuario '{}' solicitó la lista de AGENTES.", claims.sub);
    
    let service = AgentService::new(state.repo.clone(), state.ai_service.clone(), "./config");
    let agents = service.list_available_agents();
    
    info!("   -> Se devolvieron {} agentes disponibles.", agents.len());
    
    Ok(Json(agents))
}

#[utoipa::path(
    get,
    path = "/api/tools",
    responses((status = 200, body = Vec<ToolDefinition>))
)]
pub async fn list_tools(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ToolDefinition>>, AppError> {
    
    info!("🧰 ACCESO: Usuario '{}' solicitó la lista de HERRAMIENTAS.", claims.sub);

    let service = AgentService::new(state.repo.clone(), state.ai_service.clone(), "./config");
    let tools = service.list_available_tools();

    info!("   -> Se devolvieron {} herramientas disponibles.", tools.len());

    Ok(Json(tools))
}

#[utoipa::path(
    post,
    path = "/api/agents/chat",
    request_body = AgentChatRequest,
    responses((status = 200, body = AgentChatResponse))
)]
pub async fn chat_agent(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>, 
    Json(payload): Json<AgentChatRequest>,
) -> Result<Json<AgentChatResponse>, AppError> {
    
    info!("💬 CHAT: Usuario '{}' envió mensaje al Agente '{}'", claims.sub, payload.agent_id);
    info!("   📝 Mensaje: \"{}\"", payload.message);
    
    let service = AgentService::new(state.repo.clone(), state.ai_service.clone(), "./config");
    
    // Ejecutamos el agente (los logs internos de herramientas saldrán desde el executor.rs)
    let response = service.run_agent(&claims.sub, payload).await?;
    
    info!("   🤖 Respuesta generada ({} chars).", response.response.len());
    
    Ok(Json(response))
}


