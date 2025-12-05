// FILE: src/interface/handlers/chat.rs

use axum::{Json, extract::State};
use std::sync::Arc;
use rig::{
    completion::Prompt, 
    providers::openai::{self, OpenAIResponsesExt},
    client::CompletionClient 
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
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
    
    // 1. Obtener lock de lectura del servicio IA
    let ai_guard = state.ai_service.read().await;

    // 2. Generar Embedding de la pregunta del usuario
    let embedding = ai_guard.generate_embedding(&payload.message).await?;
    
    // 3. Recuperación Híbrida en Neo4j (Vector Search + Graph Traversals)
    // Traemos los top 5 fragmentos más relevantes
    let hybrid_contexts = state.repo.find_hybrid_context(embedding, 5).await?;
    
    // 4. Construir Contexto Estructurado para el Prompt y para la Respuesta API
    let mut context_text = String::new();
    let mut sources_output = Vec::new();

    for (i, ctx) in hybrid_contexts.iter().enumerate() {
        let idx = i + 1; // Índice visual 1-based (ej: [1])
        
        // Limpieza básica de espacios para ahorrar tokens y mejorar legibilidad
        let clean_content = ctx.content.replace("\n", " ").trim().to_string();
        let entity_list = ctx.connected_entities.join(", ");
        
        // Texto que leerá el LLM
        context_text.push_str(&format!(
            "FUENTE [{}]:\n- Contenido: {}\n- Conceptos Relacionados: [{}]\n\n", 
            idx, clean_content, entity_list
        ));

        // Metadatos estructurados para el Frontend (Interactividad)
        sources_output.push(SourceReference {
            index: idx,
            chunk_id: ctx.chunk_id.clone(),
            // Creamos un snippet corto para previsualización
            short_content: if clean_content.len() > 150 {
                format!("{}...", &clean_content[..150])
            } else {
                clean_content.clone()
            },
            // Simulación de relevancia (en un sistema real vendría del score vectorial)
            relevance: 1.0 - (i as f32 * 0.1), 
            concepts: ctx.connected_entities.clone(),
        });
    }

    // 5. Construcción del System Prompt
    // Es CRÍTICO instruir al modelo sobre cómo citar.
    let system_prompt = format!(
        r#"Eres 'La Muralla', un asistente de inteligencia cognitiva avanzado que responde basándose en un Grafo de Conocimiento.
        
        INSTRUCCIONES PRINCIPALES:
        1. Responde a la pregunta del usuario basándote EXCLUSIVAMENTE en las FUENTES proporcionadas abajo.
        2. NO utilices conocimiento externo si no está respaldado por el contexto.
        3. CITA SIEMPRE las fuentes al final de cada afirmación usando el formato [n], donde n es el número de la fuente.
           - Ejemplo: "El paciente presenta fiebre alta [1] y fatiga crónica [2]."
        4. Si combinas información de varias fuentes, usa [1][3].
        5. Usa formato Markdown para estructurar la respuesta (negritas, listas, encabezados).
        6. Si el contexto es insuficiente, dilo claramente.
        
        CONTEXTO RECUPERADO:
        {}
        "#, 
        context_text
    );

    // 6. Configuración dinámica del cliente LLM (Rig + Reqwest)
    let config = ai_guard.get_config(); 
    let base_url = config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
    let api_key = config.api_key.expose_secret();

    // Construcción manual del cliente HTTP para asegurar headers correctos
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    if !api_key.is_empty() {
        if let Ok(mut val) = HeaderValue::from_str(&format!("Bearer {}", api_key)) {
            val.set_sensitive(true);
            headers.insert(AUTHORIZATION, val);
        }
    }

    let client = openai::Client::from_parts(
        base_url.to_string(),
        headers,
        reqwest::Client::new(),
        OpenAIResponsesExt,
    );

    // 7. Generación de respuesta
    let agent = client.agent(&config.model_name)
        .preamble(&system_prompt)
        .build();

    let answer = agent.prompt(&payload.message).await
        .map_err(|e| AppError::AIError(format!("Error generando respuesta LLM: {}", e)))?;

    // 8. Retorno estructurado
    Ok(Json(ChatResponse {
        response: answer,
        sources: sources_output,
    }))
}