use bigdecimal::BigDecimal;
use uuid::Uuid;
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct OrderItem {
    pub id:           Uuid,
    pub order_id:     Uuid,
    pub product_id:   Uuid,
    pub product_name: String,
    pub product_type: String,
    pub unit_price:   BigDecimal,
    pub quantity:     BigDecimal,
    pub unit:         String,
    pub subtotal:     BigDecimal,
}