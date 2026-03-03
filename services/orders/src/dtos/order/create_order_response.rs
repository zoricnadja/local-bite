use serde::Serialize;
use crate::dtos::order::order_response::OrderResponse;

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub orders: Vec<OrderResponse>,
}