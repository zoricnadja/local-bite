use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use uuid::Uuid;
#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct ProductionBatch {
    pub id: Uuid,
    pub business_id: Uuid,
    pub name: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub outputs: serde_json::Value,
    pub output_name: Option<String>,
    pub output_type: Option<String>,
    pub output_unit: Option<String>,
    pub output_quantity: Option<f64>,
    pub output_expiry_date: Option<chrono::NaiveDate>,
    pub status: String,
    pub notes: Option<String>,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
