use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{
    ports::{KGRepository, AIService},
    // models::IngestionRequest, // Comentado para evitar warning
    errors::AppError
};

// Reducir drásticamente para mejorar la precisión vectorial
// 1500 caracteres ~= 300-400 tokens (Sweet spot para embeddings)
const CHUNK_SIZE: usize = 1500; 
const CHUNK_OVERLAP: usize = 200;

pub struct IngestionService {
    repo: Arc<dyn KGRepository>,
    ai: Arc<RwLock<dyn AIService>>,
}

impl IngestionService {
    pub fn new(repo: Arc<dyn KGRepository>, ai: Arc<RwLock<dyn AIService>>) -> Self {
        Self { repo, ai }
    }

    /// Función auxiliar para dividir texto preservando palabras completas
    // En split_text_into_chunks:
    // Implementar lógica de ventana deslizante (sliding window)
    fn split_text_into_chunks(&self, text: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut start = 0;

        while start < chars.len() {
            let end = std::cmp::min(start + CHUNK_SIZE, chars.len());
        
            // Ajuste para no cortar palabras (buscar espacio hacia atrás)
            let mut actual_end = end;
            if actual_end < chars.len() {
                while actual_end > start && !chars[actual_end].is_whitespace() {
                    actual_end -= 1;
                }
            }
            if actual_end == start { actual_end = end; } // Fallback si la palabra es gigante

            let chunk_str: String = chars[start..actual_end].iter().collect();
            chunks.push(chunk_str);

            // Avanzar restando el overlap para mantener contexto
            start +=  std::cmp::max(1, (actual_end - start).saturating_sub(CHUNK_OVERLAP));
        }
        chunks
    }

    pub async fn ingest_with_progress(
        &self, 
        content: String,
        progress_tx: tokio::sync::mpsc::Sender<String>
    ) -> Result<Uuid, AppError> {
        
        // 1. Dividir el contenido en trozos (Chunks)
        let chunks = self.split_text_into_chunks(&content);
        let total_chunks = chunks.len();
        let doc_group_id = Uuid::new_v4(); // ID para agrupar (opcional en lógica futura)

        let _ = progress_tx.send(format!("🔪 Documento largo detectado. Dividido en {} fragmentos.", total_chunks)).await;

        // 2. Procesar cada chunk
        for (index, chunk_text) in chunks.iter().enumerate() {
            let current_step = index + 1;
            let chunk_id = Uuid::new_v4();

            // A. Vectorizar
            let _ = progress_tx.send(format!("🧠 [{}/{}] Generando Embeddings...", current_step, total_chunks)).await;
            
            // Obtenemos lock para IA
            let ai_guard = self.ai.read().await;
            
            // Manejo de error específico de Embeddings para no detener todo el proceso si uno falla
            let embedding = match ai_guard.generate_embedding(chunk_text).await {
                Ok(emb) => emb,
                Err(e) => {
                    let _ = progress_tx.send(format!("⚠️ Error embedding chunk {}: {}. Saltando...", current_step, e)).await;
                    continue; 
                }
            };

            // B. Guardar Chunk
            // let _ = progress_tx.send(format!("💾 [{}/{}] Guardando datos...", current_step, total_chunks)).await;
            self.repo.save_chunk(chunk_id, chunk_text, embedding).await?;

            // C. Extracción Simbólica (LLM)
            let _ = progress_tx.send(format!("🕵️ [{}/{}] Extrayendo conocimiento...", current_step, total_chunks)).await;
            
            match ai_guard.extract_knowledge(chunk_text).await {
                Ok(extraction) => {
                    let count = extraction.entities.len();
                    let _ = progress_tx.send(format!("🕸️ [{}/{}] Conectando {} entidades al grafo...", current_step, total_chunks, count)).await;
                    self.repo.save_graph(chunk_id, extraction).await?;
                },
                Err(e) => {
                    let _ = progress_tx.send(format!("⚠️ Error extrayendo entidades en parte {}: {}", current_step, e)).await;
                    // No detenemos el proceso, solo avisamos
                }
            };
        }

        let _ = progress_tx.send("✅ ¡Todo el documento ha sido procesado!".to_string()).await;

        // Retornamos el ID del último chunk procesado (o uno nuevo genérico)
        Ok(doc_group_id)
    }
}