use crate::dtos::order::order_response::OrderResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub orders: Vec<OrderResponse>,
}
