use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProcessStepRequest {
    pub step_order:     i32,
    pub name:           String,
    pub description:    Option<String>,
    pub duration_hours: Option<f64>,
    pub temperature:    Option<f64>,
}