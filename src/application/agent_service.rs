use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::ports::{KGRepository, AIService};
use crate::infrastructure::persistence::agent_repo::FileAgentRepository;
// Importamos los slots generados
use crate::infrastructure::tools::executor::{
    ToolSlot0, ToolSlot1, ToolSlot2, ToolSlot3, ToolSlot4,
    ToolSlot5, ToolSlot6, ToolSlot7, ToolSlot8, ToolSlot9
};
use crate::domain::models::{AgentChatRequest, AgentChatResponse, ToolType, MessageRole, ToolDefinition};
use crate::domain::errors::AppError;
use rig::providers::openai;
use rig::completion::{Chat, Message}; 
use secrecy::ExposeSecret;
use tracing::{info, warn};

pub struct AgentService {
    kg_repo: Arc<dyn KGRepository>,
    ai_service: Arc<RwLock<dyn AIService>>,
    agent_repo: FileAgentRepository,
}

impl AgentService {
    pub fn new(
        kg_repo: Arc<dyn KGRepository>, 
        ai_service: Arc<RwLock<dyn AIService>>,
        config_path: &str
    ) -> Self {
        Self {
            kg_repo,
            ai_service,
            agent_repo: FileAgentRepository::new(config_path),
        }
    }

    // Función auxiliar para limpiar la salida ReAct (Final Answer)
    fn clean_react_output(response: &str) -> String {
        // Busca la última aparición de "Final Answer:" (o "final answer:")
        let tag = "Final Answer:";
        let tag_lower = "final answer:";
        
        // Buscamos el índice, ignorando mayúsculas
        let mut index = None;
        if let Some(i) = response.rfind(tag) { index = Some(i + tag.len()); }
        else if let Some(i) = response.rfind(tag_lower) { index = Some(i + tag_lower.len()); }

        if let Some(i) = index {
            // Devolvemos el texto que sigue al tag, limpiamos whitespace y tags de Markdown
            response[i..].trim().trim_matches('`').to_string()
        } else {
            // Si el LLM no usó el tag (por ejemplo, en un ciclo simple), devolvemos el texto completo
            response.to_string()
        }
    }

    pub fn list_available_agents(&self) -> Vec<crate::domain::models::AgentConfig> {
        self.agent_repo.list_agents()
    }

    pub fn list_available_tools(&self) -> Vec<ToolDefinition> {
        self.agent_repo.list_tools()
    }

    pub async fn run_agent(&self, username: &str, req: AgentChatRequest) -> Result<AgentChatResponse, AppError> {
        let agent_config = self.agent_repo.get_agent(&req.agent_id)?;
        self.kg_repo.save_chat_message(username, &req.agent_id, MessageRole::User, &req.message).await?;
        
        // --- RAG AUTOMÁTICO ---
        info!("🧠 [RAG] Generando embedding...");
        let embedding = {
            let ai_guard = self.ai_service.read().await;
            ai_guard.generate_embedding(&req.message).await?
        };

        let context_docs = self.kg_repo.find_hybrid_context(embedding, 3).await?;
        let mut context_str = String::new();
        if !context_docs.is_empty() {
            info!("✅ [RAG] {} fragmentos encontrados.", context_docs.len());
            context_str.push_str("\n\n### CONTEXTO (BASE DE DATOS):\n");
            for (_i, doc) in context_docs.iter().enumerate() {
                context_str.push_str(&format!("- \"{}\"\n", doc.content.trim()));
            }
            context_str.push_str("---\n");
        } else {
            warn!("⚠️ [RAG] Sin contexto relevante.");
        }
        
        let enhanced_system_prompt = format!("{}{}", agent_config.system_prompt, context_str);

        // --- PREPARACIÓN DE AGENTE Y HERRAMIENTAS (SLOTS) ---
        let ai_guard = self.ai_service.read().await;
        let ai_config = ai_guard.get_config();
        let client = openai::Client::new(ai_config.api_key.expose_secret());
        let model = agent_config.model.unwrap_or(ai_config.model_name);
        
        let mut builder = client.agent(&model).preamble(&enhanced_system_prompt);

        let tools_list = agent_config.tools;
        let total_tools = tools_list.len();

        for (i, tool_id) in tools_list.into_iter().enumerate() {
            if let Ok(tool_def) = self.agent_repo.get_tool(&tool_id) {
                let repo_ref = if matches!(tool_def.implementation, ToolType::Cypher(_)) {
                    Some(self.kg_repo.clone())
                } else { None };

                // Asignación a slots estáticos
                match i {
                    0 => builder = builder.tool(ToolSlot0::new(tool_def, repo_ref)),
                    1 => builder = builder.tool(ToolSlot1::new(tool_def, repo_ref)),
                    2 => builder = builder.tool(ToolSlot2::new(tool_def, repo_ref)),
                    3 => builder = builder.tool(ToolSlot3::new(tool_def, repo_ref)),
                    4 => builder = builder.tool(ToolSlot4::new(tool_def, repo_ref)),
                    5 => builder = builder.tool(ToolSlot5::new(tool_def, repo_ref)),
                    6 => builder = builder.tool(ToolSlot6::new(tool_def, repo_ref)),
                    7 => builder = builder.tool(ToolSlot7::new(tool_def, repo_ref)),
                    8 => builder = builder.tool(ToolSlot8::new(tool_def, repo_ref)),
                    9 => builder = builder.tool(ToolSlot9::new(tool_def, repo_ref)),
                    _ => warn!("⚠️ Límite de 10 herramientas por agente excedido. Ignorando {}", tool_id),
                }
            }
        }

        // --- EJECUCIÓN Y LIMPIEZA ---
        info!("🤖 [Agent] Ejecutando '{}' con {} herramientas activas...", model, total_tools);

        // Historial
        let history_db = self.kg_repo.get_conversation_history(username, &req.agent_id, 10).await?;
        let chat_history: Vec<Message> = history_db.iter().map(|msg| {
            Message { role: msg.role.to_string(), content: msg.content.clone() }
        }).collect();

        // 8. Ejecutar Chat (Recibe el texto crudo: Thought, Observation, Final Answer)
        let raw_response = builder.build()
            .chat(&req.message, chat_history).await
            .map_err(|e| AppError::AIError(format!("Agent execution failed: {}", e)))?;

        // 9. Limpieza
        let final_response_text = Self::clean_react_output(&raw_response); 

        // 10. Guardar respuesta limpia en memoria
        self.kg_repo.save_chat_message(username, &req.agent_id, MessageRole::Assistant, &final_response_text).await?;

        info!("   🤖 Respuesta generada ({} chars).", final_response_text.len());

        Ok(AgentChatResponse {
            response: final_response_text, 
            used_tools: vec![], 
        })
    }
}