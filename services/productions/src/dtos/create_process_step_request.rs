use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProcessStepRequest {
    pub step_order: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub variables: Vec<crate::models::process_step::StepVariable>,
}
