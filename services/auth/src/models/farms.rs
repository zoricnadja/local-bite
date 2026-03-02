use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Farm {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
}