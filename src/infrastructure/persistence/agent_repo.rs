use std::fs;
use glob::glob;
use crate::domain::models::{AgentConfig, ToolDefinition};
use crate::domain::errors::AppError;

pub struct FileAgentRepository {
    base_path: String,
}

impl FileAgentRepository {
    pub fn new(base_path: &str) -> Self {
        // Aseguramos que la ruta no termine en slash para evitar dobles slashes //
        Self { base_path: base_path.trim_end_matches('/').to_string() }
    }

    pub fn get_agent(&self, id: &str) -> Result<AgentConfig, AppError> {
        let path = format!("{}/agents/{}.yaml", self.base_path, id);
        let content = fs::read_to_string(&path)
            .map_err(|_| AppError::ConfigError(format!("Agent file not found: {}", path)))?;
        
        let agent: AgentConfig = serde_yaml::from_str(&content)
            .map_err(|e| AppError::ParseError(format!("YAML Error in {}: {}", path, e)))?;
        Ok(agent)
    }

    pub fn get_tool(&self, id: &str) -> Result<ToolDefinition, AppError> {
        let path = format!("{}/tools/{}.yaml", self.base_path, id);
        let content = fs::read_to_string(&path)
            .map_err(|_| AppError::ConfigError(format!("Tool file not found: {}", path)))?;
        
        let tool: ToolDefinition = serde_yaml::from_str(&content)
            .map_err(|e| AppError::ParseError(format!("YAML Error in {}: {}", path, e)))?;
        Ok(tool)
    }
    
    pub fn list_agents(&self) -> Vec<AgentConfig> {
        let pattern = format!("{}/agents/*.yaml", self.base_path);
        let mut agents = Vec::new();
        
        tracing::info!("🔍 Buscando agentes en: {}", pattern);

        match glob(&pattern) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            match fs::read_to_string(&path) {
                                Ok(content) => {
                                    match serde_yaml::from_str::<AgentConfig>(&content) {
                                        Ok(agent) => agents.push(agent),
                                        Err(e) => tracing::error!("❌ Error de formato YAML en {:?}: {}", path, e),
                                    }
                                },
                                Err(e) => tracing::error!("❌ No se pudo leer el archivo {:?}: {}", path, e),
                            }
                        },
                        Err(e) => tracing::error!("❌ Error accediendo al archivo glob: {}", e),
                    }
                }
            },
            Err(e) => tracing::error!("❌ Error fatal en patrón glob: {}", e),
        }
        
        tracing::info!("✅ Se encontraron {} agentes válidos.", agents.len());
        agents
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        let pattern = format!("{}/tools/*.yaml", self.base_path);
        let mut tools = Vec::new();
        
        tracing::info!("🔍 Buscando herramientas en: {}", pattern);

        match glob(&pattern) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            if let Ok(content) = fs::read_to_string(&path) {
                                match serde_yaml::from_str::<ToolDefinition>(&content) {
                                    Ok(tool) => tools.push(tool),
                                    Err(e) => tracing::error!("❌ Error de formato YAML en {:?}: {}", path, e),
                                }
                            }
                        },
                        _ => {}
                    }
                }
            },
            Err(e) => tracing::error!("❌ Error fatal en patrón glob: {}", e),
        }

        tracing::info!("✅ Se encontraron {} herramientas válidas.", tools.len());
        tools
    }
}
