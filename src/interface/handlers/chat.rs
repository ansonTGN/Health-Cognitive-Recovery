use axum::{Json, extract::State};
use std::sync::Arc;
use rig::{
    completion::Prompt, 
    providers::openai::{self}, 
    client::CompletionClient 
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
        (status = 200, description = "Respuesta RAG Estructurada con Fuentes", body = ChatResponse),
        (status = 500, description = "Error interno")
    ),
    tag = "chat"
)]
pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    
    let ai_guard = state.ai_service.read().await;

    // 1. Embedding
    let embedding = ai_guard.generate_embedding(&payload.message).await?;
    
    // 2. Retrieval
    let hybrid_contexts = state.repo.find_hybrid_context(embedding, 5).await?;
    
    let mut context_text = String::new();
    let mut sources_output = Vec::new();

    for (i, ctx) in hybrid_contexts.iter().enumerate() {
        let idx = i + 1;
        let clean_content = ctx.content.replace("\n", " ").trim().to_string();
        let entity_list = ctx.connected_entities.join(", ");
        
        context_text.push_str(&format!(
            "FUENTE [{}]:\n- Contenido: {}\n- Conceptos Relacionados: [{}]\n\n", 
            idx, clean_content, entity_list
        ));

        sources_output.push(SourceReference {
            index: idx,
            chunk_id: ctx.chunk_id.clone(),
            short_content: if clean_content.len() > 150 {
                format!("{}...", &clean_content[..150])
            } else {
                clean_content.clone()
            },
            relevance: 1.0 - (i as f32 * 0.1), 
            concepts: ctx.connected_entities.clone(),
        });
    }

    let system_prompt = format!(
        r#"Eres 'La Muralla', un asistente de inteligencia cognitiva avanzado.
        CONTEXTO RECUPERADO:
        {}
        Responde basándote EXCLUSIVAMENTE en las fuentes y cita usando formato [n]."#, 
        context_text
    );

    let config = ai_guard.get_config(); 
    let base_url = config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
    let api_key = config.api_key.expose_secret();

    // CORRECCIÓN: Creación estándar del cliente
    let client = openai::Client::from_url(api_key, base_url);

    let agent = client.agent(&config.model_name)
        .preamble(&system_prompt)
        .build();

    let answer = agent.prompt(&payload.message).await
        .map_err(|e| AppError::AIError(format!("Error generando respuesta LLM: {}", e)))?;

    Ok(Json(ChatResponse {
        response: answer,
        sources: sources_output,
    }))
}