use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct RawMaterial {
    pub id: Uuid,
    pub business_id: Uuid,
    pub name: String,
    pub material_type: String,
    pub quantity: BigDecimal,
    pub unit: String,
    pub supplier: Option<String>,
    pub origin: Option<String>,
    pub received_date: Option<NaiveDate>,
    pub harvest_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub low_stock_threshold: Option<BigDecimal>,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
