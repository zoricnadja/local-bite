use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct UpdateParams {
    pub name: String,
    pub product_type: String,
    pub description: Option<String>,
    pub quantity: BigDecimal,
    pub unit: String,
    pub price: BigDecimal,
    pub batch_id: Option<Uuid>,
    pub is_active: bool,
}