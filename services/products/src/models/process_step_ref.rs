use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessStepRef {
    pub id: Uuid,
    pub step_order: i32,
    pub name: String,
    pub description: Option<String>,
    pub duration_hours: Option<f64>,
    pub temperature: Option<f64>,
}
