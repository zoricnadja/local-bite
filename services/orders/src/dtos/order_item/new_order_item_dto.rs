use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewOrderItem {
    pub product_id:   Uuid,
    pub product_name: String,
    pub product_type: String,
    pub unit_price:   BigDecimal,
    pub quantity:     BigDecimal,
    pub unit:         String,
}