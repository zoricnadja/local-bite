use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct UpdateParams {
    pub name: String,
    pub product_type: String,
    pub description: Option<String>,
    pub quantity: Option<BigDecimal>,
    pub unit: String,
    pub price: BigDecimal,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub batch_id: Option<Uuid>,
    pub is_active: bool,
}
