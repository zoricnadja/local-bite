use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, serde::Serialize, Clone)]
pub struct OrderItemRequest {
    pub product_id: Uuid,
    pub quantity: f64,
}
