use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name:         String,
    pub product_type: String,
    pub description:  Option<String>,
    pub quantity:     f64,
    pub unit:         String,
    pub price:        f64,
    pub batch_id:     Option<Uuid>,
}


