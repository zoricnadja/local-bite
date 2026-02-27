use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AdjustQuantityRequest {
    /// Positive = restock, negative = usage/deduction
    pub delta:  f64,
    pub reason: Option<String>,
}