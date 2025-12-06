use async_trait::async_trait;
use rig::{
    providers::openai::{self},
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
};
use secrecy::ExposeSecret;
use serde_json::from_str;
use crate::domain::{models::{AIConfig, KnowledgeExtraction, InferenceResult}, ports::AIService, errors::AppError};

pub struct RigAIService {
    config: AIConfig,
}

impl RigAIService {
    pub fn new(config: AIConfig) -> Self {
        Self { config }
    }

    fn clean_json_response(&self, raw: &str) -> String {
        raw.trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .to_string()
    }
    
    fn get_client(&self) -> openai::Client {
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let api_key = self.config.api_key.expose_secret();
        openai::Client::from_url(api_key, base_url)
    }
}

#[async_trait]
impl AIService for RigAIService {
    fn update_config(&mut self, config: AIConfig) -> Result<(), AppError> {
        self.config = config;
        Ok(())
    }

    fn get_config(&self) -> AIConfig {
        self.config.clone()
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let client = self.get_client(); 
        let model = client.embedding_model(&self.config.embedding_model);
        
        let embeddings = EmbeddingsBuilder::new(model)
            .document(text) 
            .map_err(|e| AppError::AIError(format!("Error adding document: {}", e)))? 
            .build()
            .await
            .map_err(|e| AppError::AIError(format!("Embedding failed: {}", e)))?;

        let (_, embedding_data) = embeddings.first()
            .ok_or_else(|| AppError::AIError("No embedding returned".to_string()))?;
            
        let first_embedding = embedding_data.first();
        let embedding_f32: Vec<f32> = first_embedding.vec.iter().map(|&x| x as f32).collect();
        
        Ok(embedding_f32)
    }

    async fn extract_knowledge(&self, text: &str) -> Result<KnowledgeExtraction, AppError> {
        let client = self.get_client(); 

        let ontology_prompt = r#"
        Eres un experto en Psicología. Extrae entidades (Person, Condition, Intervention, Outcome) y relaciones.
        Output JSON: { "entities": [...], "relations": [...] }
        "#;

        let agent = client.agent(&self.config.model_name)
            .preamble(ontology_prompt)
            .build();

        let response = agent.prompt(text).await
            .map_err(|e| AppError::AIError(format!("Extraction failed: {}", e)))?;

        let cleaned_json = self.clean_json_response(&response);
        let extraction: KnowledgeExtraction = from_str(&cleaned_json)
            .map_err(|e| AppError::ParseError(format!("JSON Error: {}", e)))?;

        Ok(extraction)
    }

    async fn generate_inference(&self, prompt: &str) -> Result<InferenceResult, AppError> {
        let client = self.get_client();
        let agent = client.agent(&self.config.model_name).build();
        
        let response = agent.prompt(prompt).await
            .map_err(|e| AppError::AIError(format!("Inference failed: {}", e)))?;
            
        let cleaned = self.clean_json_response(&response);
        let result: InferenceResult = serde_json::from_str(&cleaned)
            .map_err(|e| AppError::ParseError(format!("JSON Error: {}", e)))?;
            
        Ok(result)
    }
}