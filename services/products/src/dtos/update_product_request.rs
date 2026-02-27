use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name:         Option<String>,
    pub product_type: Option<String>,
    pub description:  Option<String>,
    pub quantity:     Option<f64>,
    pub unit:         Option<String>,
    pub price:        Option<f64>,
    pub batch_id:     Option<Uuid>,
    pub is_active:    Option<bool>,
}