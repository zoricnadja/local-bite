use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DecrementRequest {
    pub quantity: f64,
}