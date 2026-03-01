use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct InsertParams {
    pub id: Uuid,
    pub farm_id: Uuid,
    pub name: String,
    pub product_type: String,
    pub description: Option<String>,
    pub quantity: BigDecimal,
    pub unit: String,
    pub price: BigDecimal,
    pub batch_id: Option<Uuid>,
    pub qr_token: Uuid,
    pub qr_path: Option<String>,
}