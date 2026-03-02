use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct OrderItemRequest {
    pub product_id: Uuid,
    pub quantity:   f64,
}