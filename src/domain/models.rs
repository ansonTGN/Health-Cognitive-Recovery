use serde::{Deserialize, Serialize};
use secrecy::SecretString;
use utoipa::{ToSchema, IntoParams}; 
use validator::Validate;
use std::fmt;

// --- 1. SEGURIDAD & USUARIOS (NUEVO) ---

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub enum UserRole {
    Admin,
    User,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::User => write!(f, "User"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,     // Subject (Username)
    pub role: UserRole,  // RBAC Role
    pub exp: usize,      // Expiration
}

// --- 2. CONFIGURACIÓN IA ---

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

// --- 3. CORE DEL GRAFO (GraphRAG) ---

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

// --- 4. VISUALIZACIÓN ---

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

// --- 5. CHAT & RAG ---

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SourceReference {
    pub index: usize,
    pub chunk_id: String,
    pub short_content: String,
    pub relevance: f32,
    pub concepts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChatResponse {
    pub response: String,
    pub sources: Vec<SourceReference>,
}

#[derive(Debug, Clone)]
pub struct HybridContext {
    pub chunk_id: String,
    pub content: String,
    pub connected_entities: Vec<String>, 
}

// --- 6. INFERENCIA & EXPORTACIÓN ---

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportedGraph {
    pub generated_at: String,
    pub domain: String,
    pub nodes: Vec<GraphEntity>,
    pub edges: Vec<GraphRelation>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub enum ExportFormat {
    #[serde(rename = "jsonld")]
    JsonLd,
    #[serde(rename = "turtle")]
    Turtle,
    #[serde(rename = "graphml")]
    GraphML,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)] 
pub struct ExportParams {
    pub format: Option<ExportFormat>,
}