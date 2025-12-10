use rig::tool::Tool;
use rig::completion::ToolDefinition as RigToolDefinition;
use serde_json::Value;
use std::sync::Arc;
use tokio::task; 
use tracing::{info, warn, error}; 
use url::form_urlencoded;

use crate::domain::models::{ToolDefinition, ToolType};
use crate::domain::ports::KGRepository;
use crate::domain::errors::AppError;

// Lógica común de ejecución (sin cambios)
async fn execute_common(
    definition: &ToolDefinition, 
    kg_repo: &Option<Arc<dyn KGRepository>>, 
    args: Value
) -> Result<String, AppError> {
    
    match &definition.implementation {
        ToolType::Http(config) => {
            info!("🌐 TOOL EXEC: HTTP {} solicitado a {}", config.method, config.url);
            let client = reqwest::Client::new();
            let mut url = config.url.clone();
            if let Some(obj) = args.as_object() {
                for (k, v) in obj {
                    let key = format!("{{{{input.{}}}}}", k);
                    let val = v.as_str().unwrap_or(&v.to_string()).to_string();
                    let encoded_val: String = form_urlencoded::byte_serialize(val.as_bytes()).collect();
                    url = url.replace(&key, &encoded_val);
                }
            }
            let mut req = match config.method.to_uppercase().as_str() {
                "POST" => client.post(&url),
                "PUT" => client.put(&url),
                _ => client.get(&url),
            };
            if let Some(_body_tmpl) = &config.body_template {
                 req = req.json(&args);
            }
            let res = req.send().await.map_err(|e| AppError::AIError(e.to_string()))?;
            let status = res.status();
            let text = res.text().await.map_err(|e| AppError::AIError(e.to_string()))?;
            info!("🌐 TOOL RES: HTTP Status {}", status);
            Ok(text)
        },
        ToolType::Cypher(_) => {
            info!("🛠️ TOOL EXEC: Cypher solicitado. Args: {:?}", args);
            if let Some(repo) = kg_repo {
                let repo_arc = repo.clone();

                if args.get("action").and_then(|v| v.as_str()) == Some("get_schema") {
                    let handle: task::JoinHandle<Result<String, AppError>> = task::spawn(async move {
                        repo_arc.get_graph_schema().await
                    });
                    return match handle.await {
                        Ok(res) => res.map_err(|e| AppError::DatabaseError(e.to_string())),
                        Err(e) => Err(AppError::DatabaseError(format!("Task Join Error: {}", e))),
                    };
                }

                if let Some(query_str) = args.get("query").and_then(|v| v.as_str()) {
                    let query_string = query_str.to_string();
                    let handle: task::JoinHandle<Result<String, AppError>> = task::spawn(async move {
                        repo_arc.execute_cypher_query(&query_string).await
                    });
                    return match handle.await {
                        Ok(res) => res.map_err(|e| AppError::DatabaseError(e.to_string())),
                        Err(e) => Err(AppError::DatabaseError(format!("Task Join Error: {}", e))),
                    };
                }

                if let Some(concept) = args.get("concept_name").and_then(|v| v.as_str()) {
                    let concept_string = concept.to_string();
                    let concept_key = concept_string.clone(); 
                    let handle = task::spawn(async move {
                        let data = repo_arc.get_concept_neighborhood(&concept_key).await?;
                        Ok::<String, AppError>(serde_json::to_string(&data).unwrap_or_default())
                    });
                    
                    return match handle.await {
                        Ok(Ok(json_res)) => {
                            if json_res.len() < 50 { 
                                warn!("⚠️ TOOL: Graph Explorer devolvió vacío para '{}'", concept_string);
                                return Ok(format!("No se encontraron datos para '{}'.", concept_string));
                            }
                            info!("✅ TOOL: Datos encontrados para '{}' ({} chars)", concept_string, json_res.len());
                            Ok(json_res)
                        },
                        Ok(Err(e)) => Err(AppError::DatabaseError(e.to_string())),
                        Err(e) => Err(AppError::DatabaseError(format!("Task Join Error: {}", e))),
                    };
                }
            } else {
                error!("🔥 TOOL ERROR: Repositorio no inyectado.");
            }
            Ok("Error de configuración de herramienta Cypher.".to_string())
        },
        ToolType::Cli(_) => Err(AppError::ConfigError("CLI tools disabled".into())),
    }
}

// =========================================================
// MACRO PARA GENERAR SLOTS DE HERRAMIENTAS
// =========================================================
macro_rules! define_tool_slot {
    ($struct_name:ident, $tool_name_const:literal) => {
        pub struct $struct_name {
            pub definition: ToolDefinition,
            pub kg_repo: Option<Arc<dyn KGRepository>>,
        }

        impl $struct_name {
            pub fn new(definition: ToolDefinition, kg_repo: Option<Arc<dyn KGRepository>>) -> Self {
                Self { definition, kg_repo }
            }
        }

        impl Tool for $struct_name {
            // El nombre interno que Rig usa para enrutar
            const NAME: &'static str = $tool_name_const;

            type Error = AppError;
            type Args = Value;
            type Output = String;

            async fn definition(&self, _prompt: String) -> RigToolDefinition {
                // TRUCO: Le decimos al LLM que la herramienta se llama igual que el slot ($tool_name_const)
                // pero en la descripción le ponemos el nombre real ("Weather Service") para que sepa qué hace.
                RigToolDefinition {
                    name: $tool_name_const.to_string(), 
                    description: format!("{} (INTERNAL_ID: {}) - {}", self.definition.name, $tool_name_const, self.definition.description),
                    parameters: self.definition.input_schema.clone(),
                }
            }

            async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
                execute_common(&self.definition, &self.kg_repo, args).await
            }
        }
    };
}

// Generamos 10 slots (tool_0 hasta tool_9)
// Esto permite cargar hasta 10 herramientas dinámicas por agente.
define_tool_slot!(ToolSlot0, "tool_0");
define_tool_slot!(ToolSlot1, "tool_1");
define_tool_slot!(ToolSlot2, "tool_2");
define_tool_slot!(ToolSlot3, "tool_3");
define_tool_slot!(ToolSlot4, "tool_4");
define_tool_slot!(ToolSlot5, "tool_5");
define_tool_slot!(ToolSlot6, "tool_6");
define_tool_slot!(ToolSlot7, "tool_7");
define_tool_slot!(ToolSlot8, "tool_8");
define_tool_slot!(ToolSlot9, "tool_9");