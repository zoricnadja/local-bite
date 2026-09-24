use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessStepRef {
    pub id: Uuid,
    pub step_order: i32,
    pub status: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub variables: Vec<StepVariableRef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StepVariableRef {
    pub name: String,
    pub value: String,
}
