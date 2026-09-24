use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProcessStepResponse {
    pub id: Uuid,
    pub step_order: i32,
    pub status: String,
    pub name: String,
    pub description: Option<String>,
    pub variables: serde_json::Value,
}
