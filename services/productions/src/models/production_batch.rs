use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use uuid::Uuid;
#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct ProductionBatch {
    pub id:           Uuid,
    pub farm_id:      Uuid,
    pub name:         String,
    pub process_type: String,
    pub start_date:   Option<NaiveDate>,
    pub end_date:     Option<NaiveDate>,
    pub status:       String,
    pub notes:        Option<String>,
    pub is_deleted:   bool,
    pub created_at:   NaiveDateTime,
    pub updated_at:   NaiveDateTime,
}