use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::models::AIConfig;

#[derive(Deserialize, ToSchema)]
pub struct AdminConfigPayload {
    pub config: AIConfig,
    pub force_reset: bool,
}

// Puedes eliminar IngestionResponse si no lo usas, o dejarlo así (el warning no afecta)
#[derive(Serialize, ToSchema)]
pub struct IngestionResponse {
    pub id: String,
    pub status: String,
}