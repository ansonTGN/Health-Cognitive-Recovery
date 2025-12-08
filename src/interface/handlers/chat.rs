use axum::{Json, extract::State};
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
    State(state): State<AppState>, 
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    
    // 1. Generar Embedding
    let ai_guard = state.ai_service.read().await;
    let embedding = ai_guard.generate_embedding(&payload.message).await?;
    
    // 2. Buscar contexto híbrido (Texto + Entidades del Grafo)
    // Pedimos los 5 fragmentos más relevantes
    let hybrid_contexts = state.repo.find_hybrid_context(embedding, 5).await?;
    
    let mut context_text = String::new();
    let mut sources_output = Vec::new();

    context_text.push_str("EVIDENCIA RECUPERADA DEL SISTEMA:\n");

    for (i, ctx) in hybrid_contexts.iter().enumerate() {
        let idx = i + 1;
        let clean_content = ctx.content.replace("\n", " ").trim().to_string();
        let entities_str = ctx.connected_entities.join(", ");
        
        // Inyectamos al Prompt tanto el texto como las entidades que el grafo conoce sobre este texto
        context_text.push_str(&format!(
            "FUENTE [{}]:\n- Texto: \"{}\"\n- Entidades Clave en Grafo: [{}]\n\n", 
            idx, clean_content, entities_str
        ));

        sources_output.push(SourceReference {
            index: idx,
            chunk_id: ctx.chunk_id.clone(),
            short_content: clean_content.chars().take(200).collect(),
            relevance: 1.0, // Podríamos usar score si Neo4j lo devuelve
            concepts: ctx.connected_entities.clone(),
        });
    }

    // 3. Configurar Cliente IA
    let config = ai_guard.get_config(); 
    let raw_base = config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
    let clean_base = raw_base.trim_end_matches("/v1").trim_end_matches('/');
    let api_key = config.api_key.expose_secret();
    let client = openai::Client::from_url(api_key, clean_base);

    // 4. Prompt de Sistema Profesional
    let system_prompt = format!(
        r#"
        Actúa como "LaMuralla AI", un asistente experto en análisis clínico y social.
        
        TU TAREA:
        Responder a la pregunta del usuario basándote EXCLUSIVAMENTE en las FUENTES proporcionadas.
        
        REGLAS DE RESPUESTA:
        1. Tono profesional, empático y basado en evidencia.
        2. CITAS OBLIGATORIAS: Cada afirmación debe estar respaldada por su fuente.
           - Formato: "El paciente muestra mejora en autonomía [1], relacionado con su asistencia al taller [2]."
           - Usa [n] al final de las frases.
        3. Si la información no está en las fuentes, di explícitamente: "No tengo evidencia registrada sobre esto."
        4. Usa Markdown para negritas y listas.
        
        CONTEXTO:
        {}
        "#, 
        context_text
    );

    let agent = client.agent(&config.model_name)
        .preamble(&system_prompt)
        .build();

    let answer = agent.prompt(&payload.message).await
        .map_err(|e| AppError::AIError(format!("LLM Error: {}", e)))?;

    Ok(Json(ChatResponse {
        response: answer,
        sources: sources_output,
    }))
}
