// FILE: src/domain/models.rs
use serde::{Deserialize, Serialize};
use secrecy::SecretString;
use utoipa::ToSchema;
use validator::Validate;

// --- CONFIGURACIÓN (Sin cambios significativos) ---

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub enum AIProvider {
    OpenAI,
    Ollama,
    Groq,
}

fn default_api_key() -> SecretString {
    SecretString::new("".into())
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
pub struct AIConfig {
    pub provider: AIProvider,
    #[validate(length(min = 1))]
    pub model_name: String,
    #[validate(length(min = 1))]
    pub embedding_model: String,
    
    #[serde(skip_serializing, default = "default_api_key")]
    #[schema(value_type = String)] 
    pub api_key: SecretString,
    
    pub embedding_dim: usize,
    #[validate(url)]
    pub base_url: Option<String>, 
}

// --- GRAFO BÁSICO (Sin cambios) ---

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GraphEntity {
    pub name: String,
    pub category: String, 
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GraphRelation {
    pub source: String,
    pub target: String,
    pub relation_type: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeExtraction {
    pub entities: Vec<GraphEntity>,
    pub relations: Vec<GraphRelation>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct IngestionRequest {
    #[validate(length(min = 10))]
    pub content: String,
    pub metadata: serde_json::Value,
}

// --- VISUALIZACIÓN (Sin cambios) ---

#[derive(Debug, Serialize, ToSchema)]
pub struct VisNode {
    pub id: String,
    pub label: String,
    pub group: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VisEdge {
    pub from: String,
    pub to: String,
    pub label: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GraphDataResponse {
    pub nodes: Vec<VisNode>,
    pub edges: Vec<VisEdge>,
}

// --- CHAT RAG AVANZADO (MODIFICADO) ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChatRequest {
    pub message: String,
}

/// Referencia a una fuente documental específica.
/// Se usa para crear citas interactivas [1] que iluminan el grafo.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SourceReference {
    /// Índice visual para la cita (ej: 1, 2, 3)
    pub index: usize,
    /// ID interno del chunk
    pub chunk_id: String,
    /// Fragmento de texto para mostrar en tooltip/panel
    pub short_content: String,
    /// Puntuación de relevancia (0.0 - 1.0)
    pub relevance: f32,
    /// Conceptos (nodos) del grafo presentes en este fragmento.
    /// Clave para la interactividad Visual <-> Texto.
    pub concepts: Vec<String>,
}

/// Respuesta estructurada del chat.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChatResponse {
    /// Texto generado por el LLM (Markdown)
    pub response: String,
    /// Lista de fuentes utilizadas para generar la respuesta
    pub sources: Vec<SourceReference>,
}

#[derive(Debug, Clone)]
pub struct HybridContext {
    pub chunk_id: String,
    pub content: String,
    pub connected_entities: Vec<String>, 
}

// --- RAZONAMIENTO E INFERENCIA ---

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct InferredRelation {
    pub source: String,
    pub target: String,
    pub relation: String,
    pub reasoning: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceResult {
    pub new_relations: Vec<InferredRelation>,
}

// AÑADIR en src/domain/models.rs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportedGraph {
    pub generated_at: String,
    pub domain: String,
    pub nodes: Vec<GraphEntity>,
    pub edges: Vec<GraphRelation>,
}

// Opcional: Enums para validación estricta post-extracción
#[derive(Debug, PartialEq)]
pub enum EntityCategory {
    Person,
    Condition,
    Intervention,
    Outcome,
    CommunityResource,
    Concept,
    Unknown(String),
}