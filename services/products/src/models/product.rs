use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub business_id: Uuid,
    pub name: String,
    pub product_type: String,
    pub description: Option<String>,
    pub quantity: BigDecimal,
    pub unit: String,
    pub price: BigDecimal,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub batch_id: Option<Uuid>,
    pub image_path: Option<String>,
    pub qr_token: Uuid,
    pub qr_path: Option<String>,
    pub status: String,
    pub is_active: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
