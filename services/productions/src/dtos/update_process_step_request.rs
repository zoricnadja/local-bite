use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProcessStepRequest {
    pub step_order: Option<i32>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub variables: Option<Vec<crate::models::process_step::StepVariable>>,
}
