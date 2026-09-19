use uuid::Uuid;

use rust_decimal::Decimal;

#[derive(serde::Deserialize)]
pub struct ProductApiData {

    pub name: String,
    pub product_type: String,
    pub price: Decimal,

    pub unit: String,
    pub is_active: bool,
    pub farm_id: Option<Uuid>,
}
