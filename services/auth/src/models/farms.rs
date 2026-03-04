use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Farm {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,

    // Required fields
    pub address: String,
    pub photo_url: String,

    // Optional fields
    pub phone: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}