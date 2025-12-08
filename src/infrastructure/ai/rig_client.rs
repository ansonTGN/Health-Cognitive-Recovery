use async_trait::async_trait;
use rig::{
    providers::openai::{self},
    completion::Prompt,
};
use secrecy::ExposeSecret;
use serde_json::{json, from_str, Value}; 
use crate::domain::{
    models::{AIConfig, KnowledgeExtraction, InferenceResult}, 
    ports::AIService, 
    errors::AppError
};

pub struct RigAIService {
    config: AIConfig,
    http_client: reqwest::Client,
}

impl RigAIService {
    pub fn new(config: AIConfig) -> Self {
        Self { 
            config,
            http_client: reqwest::Client::new(),
        }
    }

    fn clean_json_response(&self, raw: &str) -> String {
        let start = raw.find('{').unwrap_or(0);
        let end = raw.rfind('}').map(|i| i + 1).unwrap_or(raw.len());
        if start >= end { return raw.to_string(); }
        raw[start..end].to_string()
    }
    
    fn get_rig_client(&self) -> openai::Client {
        let raw_base = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let no_slash = raw_base.trim_end_matches('/');
        let clean_base = if no_slash.ends_with("/v1") {
            no_slash.trim_end_matches("/v1")
        } else {
            no_slash
        };
        tracing::info!("🔌 Rig URL Fix: Original='{}' -> Ajustada='{}'", raw_base, clean_base);
        let api_key = self.config.api_key.expose_secret();
        openai::Client::from_url(api_key, clean_base)
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
        if text.trim().is_empty() {
            return Err(AppError::ValidationError("Texto vacío para embedding".to_string()));
        }
        
        let api_key = self.config.api_key.expose_secret();
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/embeddings", base_url.trim_end_matches('/'));

        let response = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "input": text,
                "model": self.config.embedding_model
            }))
            .send()
            .await
            .map_err(|e| AppError::AIError(format!("Error de Red al contactar OpenAI: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::AIError(format!("OpenAI rechazó la petición: {} - {}", status, error_text)));
        }

        let body: Value = response.json().await
            .map_err(|e| AppError::ParseError(format!("Error leyendo JSON de OpenAI: {}", e)))?;

        let embedding_value = body.get("data")
            .and_then(|d| d.get(0))
            .and_then(|item| item.get("embedding"))
            .ok_or_else(|| AppError::AIError("El JSON de respuesta no contiene 'data[0].embedding'".to_string()))?;

        let vector: Vec<f32> = serde_json::from_value(embedding_value.clone())
            .map_err(|e| AppError::ParseError(format!("El embedding no es un array de floats: {}", e)))?;

        Ok(vector)
    }

    async fn extract_knowledge(&self, text: &str) -> Result<KnowledgeExtraction, AppError> {
        let client = self.get_rig_client(); 

        let ontology_prompt = r#"
        Eres un experto auditor clínico y ontólogo. Tu trabajo es estructurar texto libre en un Grafo de Conocimiento.
        
        ONTOLOGÍA ESTRICTA:
        - Person: Pacientes, profesionales, familiares.
        - Condition: Diagnósticos, síntomas, estados emocionales (ej. Ansiedad, Esquizofrenia, Soledad).
        - Intervention: Terapias, talleres, medicación, actividades (ej. Club Social, Taller de Arte).
        - Outcome: Resultados observables (ej. Mejora autoestima, Adherencia tratamiento).
        - CommunityResource: Entidades externas (ej. Ayuntamiento, Hospital, ONG).

        INSTRUCCIONES:
        1. Extrae entidades relevantes.
        2. Extrae relaciones lógicas (CAUSES, TREATED_WITH, PARTICIPATES_IN, HAS_SYMPTOM).
        3. Output JSON puro.

        Output Format:
        { 
            "entities": [{"name": "Nombre Único", "category": "Categoría"}], 
            "relations": [{"source": "Nombre1", "target": "Nombre2", "relation_type": "VERBO_CORTO"}] 
        }
        "#;

        let agent = client.agent(&self.config.model_name)
            .preamble(ontology_prompt)
            .build();

        let response = agent.prompt(text).await
            .map_err(|e| AppError::AIError(format!("Extraction Failed: {}", e)))?;

        let cleaned_json = self.clean_json_response(&response);
        let extraction: KnowledgeExtraction = from_str(&cleaned_json)
            .map_err(|e| AppError::ParseError(format!("Invalid JSON Extraction: {}", e)))?;

        Ok(extraction)
    }

    async fn generate_inference(&self, prompt: &str) -> Result<InferenceResult, AppError> {
        let client = self.get_rig_client();
        let agent = client.agent(&self.config.model_name).build();
        
        let response = agent.prompt(prompt).await
            .map_err(|e| AppError::AIError(format!("Inference failed: {}", e)))?;
            
        let cleaned = self.clean_json_response(&response);
        let result: InferenceResult = serde_json::from_str(&cleaned)
            .map_err(|e| AppError::ParseError(format!("Invalid JSON Inference: {}", e)))?;
            
        Ok(result)
    }
}