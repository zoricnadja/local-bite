use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id:           Uuid,
    pub product_id:   Uuid,
    pub product_name: String,
    pub product_type: String,
    pub unit_price:   f64,
    pub quantity:     f64,
    pub unit:         String,
    pub subtotal:     f64,
}