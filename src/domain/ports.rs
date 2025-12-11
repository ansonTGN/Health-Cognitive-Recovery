use async_trait::async_trait;
use crate::domain::models::{
    AIConfig, KnowledgeExtraction, GraphDataResponse, HybridContext, 
    InferredRelation, InferenceResult, ExportedGraph, User,
    ChatHistoryMessage, MessageRole // Importante: importar los nuevos modelos
};
use crate::domain::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait KGRepository: Send + Sync {
    // --- Capacidades Core: Grafo y Vectores ---
    async fn save_chunk(&self, id: Uuid, content: &str, embedding: Vec<f32>) -> Result<(), AppError>;
    async fn save_graph(&self, chunk_id: Uuid, data: KnowledgeExtraction) -> Result<(), AppError>;
    async fn reset_database(&self) -> Result<(), AppError>;
    async fn create_indexes(&self, dim: usize) -> Result<(), AppError>;
    
    // --- Capacidades RAG: Lectura ---
    async fn get_full_graph(&self) -> Result<GraphDataResponse, AppError>;
    async fn find_hybrid_context(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<HybridContext>, AppError>;
    async fn get_concept_neighborhood(&self, concept_name: &str) -> Result<GraphDataResponse, AppError>;

    // --- Capacidades IA: Razonamiento y Exportación ---
    async fn get_graph_context_for_reasoning(&self, limit: usize) -> Result<String, AppError>;
    async fn save_inferred_relations(&self, relations: Vec<InferredRelation>) -> Result<(), AppError>;
    async fn export_full_knowledge_graph(&self) -> Result<ExportedGraph, AppError>;  

    // --- Capacidades Seguridad: Gestión de Identidad ---
    async fn create_user(&self, user: User) -> Result<(), AppError>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn ensure_admin_exists(&self, username: &str, hash: &str) -> Result<(), AppError>;
    async fn get_all_users(&self) -> Result<Vec<User>, AppError>;
    async fn delete_user(&self, username: &str) -> Result<(), AppError>;

    // --- Capacidades de Memoria (NUEVO) ---
    async fn get_conversation_history(&self, username: &str, agent_id: &str, limit: usize) -> Result<Vec<ChatHistoryMessage>, AppError>;
    async fn save_chat_message(&self, username: &str, agent_id: &str, role: MessageRole, content: &str) -> Result<(), AppError>;
    
    // 1. Introspección: Devuelve un resumen del esquema (Nodos, Relaciones y Propiedades)
    async fn get_graph_schema(&self) -> Result<String, AppError>;

    // 2. Ejecución Dinámica: Ejecuta una query Cypher generada por la IA (Read-Only recomendado)
    async fn execute_cypher_query(&self, query: &str) -> Result<String, AppError>;
}

#[async_trait]
pub trait AIService: Send + Sync {
    async fn extract_knowledge(&self, text: &str) -> Result<KnowledgeExtraction, AppError>;
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, AppError>;
    fn update_config(&mut self, config: AIConfig) -> Result<(), AppError>;
    fn get_config(&self) -> AIConfig;
    async fn generate_inference(&self, prompt: &str) -> Result<InferenceResult, AppError>;
    // NUEVO: Capacidad de ver (Vision)
    async fn describe_image(&self, image_bytes: &[u8], mime_type: &str) -> Result<String, AppError>;
    // NUEVO: Capacidad de oír (Whisper)
    async fn transcribe_audio(&self, audio_bytes: &[u8], filename: &str) -> Result<String, AppError>;
}