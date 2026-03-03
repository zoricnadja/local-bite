use uuid::Uuid;

use rust_decimal::Decimal;

#[derive(serde::Deserialize)]
pub struct ProductApiData {
    pub id: Uuid,
    pub name: String,
    pub product_type: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub unit: String,
    pub is_active: bool,
    pub farm_id: Option<Uuid>,
}
