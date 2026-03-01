use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct ProcessStep {
    pub id:             Uuid,
    pub batch_id:       Uuid,
    pub farm_id:        Uuid,
    pub step_order:     i32,
    pub name:           String,
    pub description:    Option<String>,
    pub duration_hours: Option<BigDecimal>,
    pub temperature:    Option<BigDecimal>,
    pub created_at:     NaiveDateTime,
    pub updated_at:     NaiveDateTime,
}