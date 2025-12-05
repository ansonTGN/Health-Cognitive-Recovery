use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::models::AIConfig;

#[derive(Deserialize, ToSchema)]
pub struct AdminConfigPayload {
    pub config: AIConfig,
    pub force_reset: bool,
}

#[derive(Serialize, ToSchema)]
pub struct IngestionResponse {
    pub id: String,
    pub status: String,
}