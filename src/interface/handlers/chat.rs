use axum::{Json, extract::State};
use std::sync::Arc;
use rig::{
    completion::Prompt, 
    providers::openai::{self}
};
use secrecy::ExposeSecret; 
use crate::domain::{
    models::{ChatRequest, ChatResponse, SourceReference}, 
    errors::AppError
};
use super::admin::AppState;

#[utoipa::path(
    post,
    path = "/api/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Respuesta RAG", body = ChatResponse),
        (status = 500, description = "Error interno")
    ),
    tag = "chat"
)]
pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    
    let ai_guard = state.ai_service.read().await;

    let embedding = ai_guard.generate_embedding(&payload.message).await?;
    let hybrid_contexts = state.repo.find_hybrid_context(embedding, 5).await?;
    
    let mut context_text = String::new();
    let mut sources_output = Vec::new();

    for (i, ctx) in hybrid_contexts.iter().enumerate() {
        let idx = i + 1;
        let clean_content = ctx.content.replace("\n", " ").trim().to_string();
        context_text.push_str(&format!("FUENTE [{}]: {}\n\n", idx, clean_content));

        sources_output.push(SourceReference {
            index: idx,
            chunk_id: ctx.chunk_id.clone(),
            short_content: clean_content.chars().take(150).collect(),
            relevance: 1.0,
            concepts: ctx.connected_entities.clone(),
        });
    }

    let config = ai_guard.get_config(); 
    let base_url = config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
    let api_key = config.api_key.expose_secret();
    let client = openai::Client::from_url(api_key, base_url);

    let agent = client.agent(&config.model_name)
        .preamble(&format!("Usa este contexto para responder:\n{}", context_text))
        .build();

    let answer = agent.prompt(&payload.message).await
        .map_err(|e| AppError::AIError(format!("LLM Error: {}", e)))?;

    Ok(Json(ChatResponse {
        response: answer,
        sources: sources_output,
    }))
}