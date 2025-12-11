use axum::{
    extract::{State, Multipart},
    response::IntoResponse,
    body::{Body, Bytes},
};
use tokio::sync::mpsc;
use tokio::task; 
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use mime_guess::from_path;

use crate::application::ingestion::IngestionService;
// IMPORTANTE: Apuntamos al único transmutador válido en Infrastructure
use crate::infrastructure::transmutation::DocumentTransmuter;
use super::admin::AppState;

#[utoipa::path(
    post,
    path = "/api/ingest",
    request_body(content_type = "multipart/form-data", description = "File upload (Multimodal)", content = String), 
    responses(
        (status = 200, description = "Stream de progreso"),
        (status = 500, description = "Error interno")
    )
)]
pub async fn ingest_document(
    State(state): State<AppState>, 
    mut multipart: Multipart,
) -> impl IntoResponse {

    let (tx, rx) = mpsc::channel::<String>(20);
    let tx_inner = tx.clone();

    tokio::spawn(async move {
        let mut final_content = String::new();
        let mut content_found = false;

        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or("").to_string();

            if name == "file" {
                let filename = field.file_name().unwrap_or("archivo.bin").to_string();
                let _ = tx_inner.send(format!("📂 Archivo: {}", filename)).await;
                
                match field.bytes().await {
                    Ok(bytes) => {
                        let bytes_vec = bytes.to_vec();
                        let mime_type = from_path(&filename).first_or_octet_stream().to_string();
                        
                        let processing_result: Result<String, String> = if mime_type.starts_with("image/") {
                            // 1. VISIÓN
                            let _ = tx_inner.send("👁️ Imagen detectada. Analizando...".to_string()).await;
                            state.ai_service.read().await
                                .describe_image(&bytes_vec, &mime_type).await
                                .map(|d| format!("--- [IMG: {}] ---\n{}\n---", filename, d))
                                .map_err(|e| format!("Error Visión: {}", e))
                        } else if mime_type.starts_with("audio/") {
                            // 2. AUDIO
                            let _ = tx_inner.send("👂 Audio detectado. Transcribiendo...".to_string()).await;
                            state.ai_service.read().await
                                .transcribe_audio(&bytes_vec, &filename).await
                                .map(|t| format!("--- [AUDIO: {}] ---\n{}\n---", filename, t))
                                .map_err(|e| format!("Error Audio: {}", e))
                        } else {
                            // 3. DOCUMENTOS (Usando el transmutador unificado)
                            let _ = tx_inner.send("📄 Extrayendo texto...".to_string()).await;
                            let fname = filename.clone();
                            
                            task::spawn_blocking(move || {
                                DocumentTransmuter::transmute(&fname, &bytes_vec)
                            }).await
                            .map_err(|e| format!("Error Thread: {}", e))
                            .and_then(|res| res.map_err(|e| format!("Error Formato: {}", e)))
                        };

                        match processing_result {
                            Ok(text) => {
                                final_content.push_str(&text);
                                final_content.push_str("\n\n");
                                content_found = true;
                                let _ = tx_inner.send("✅ Contenido extraído.".to_string()).await;
                            },
                            Err(e) => { let _ = tx_inner.send(format!("❌ {}", e)).await; }
                        }
                    },
                    Err(e) => { let _ = tx_inner.send(format!("❌ Error Subida: {}", e)).await; }
                }

            } else if name == "content" {
                 if let Ok(text) = field.text().await {
                    if !text.trim().is_empty() {
                        final_content.push_str(&text);
                        content_found = true;
                    }
                 }
            }
        }

        if content_found && final_content.len() > 5 {
            let _ = tx_inner.send("🧠 Ingestando en Grafo...".to_string()).await;
            let service = IngestionService::new(state.repo.clone(), state.ai_service.clone());
            if let Err(e) = service.ingest_with_progress(final_content, tx_inner.clone()).await {
                 let _ = tx_inner.send(format!("❌ Error GraphRAG: {}", e)).await;
            } else {
                 let _ = tx_inner.send("DONE".to_string()).await;
            }
        } else {
            let _ = tx_inner.send("❌ Sin contenido válido.".to_string()).await;
        }
    });

    Body::from_stream(ReceiverStream::new(rx).map(|msg| Ok::<_, std::io::Error>(Bytes::from(format!("{}\n", msg)))))
}