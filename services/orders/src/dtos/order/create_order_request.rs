use serde::Deserialize;
use uuid::Uuid;
use crate::dtos::order_item::order_item_request::OrderItemRequest;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    /// Optional — if provided, must match a real user (CUSTOMER role).
    pub customer_id:    Option<Uuid>,
    pub customer_name:  Option<String>,
    pub customer_email: Option<String>,
    pub notes:          Option<String>,
    pub items:          Vec<OrderItemRequest>,
    pub farm_id:        Option<Uuid>,
}