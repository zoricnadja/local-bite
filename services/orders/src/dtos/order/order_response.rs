use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;
use crate::dtos::order_item::order_item_response::OrderItemResponse;

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id:             Uuid,
    pub farm_id:        Uuid,
    pub customer_id:    Option<Uuid>,
    pub customer_name:  Option<String>,
    pub customer_email: Option<String>,
    pub status:         String,
    pub total_price:    f64,
    pub notes:          Option<String>,
    pub items:          Vec<OrderItemResponse>,
    pub created_at:     NaiveDateTime,
    pub updated_at:     NaiveDateTime,
}