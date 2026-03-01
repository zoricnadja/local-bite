use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProcessStepRequest {
    pub step_order:     Option<i32>,
    pub name:           Option<String>,
    pub description:    Option<String>,
    pub duration_hours: Option<f64>,
    pub temperature:    Option<f64>,
}