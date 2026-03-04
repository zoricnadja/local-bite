use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use common::models::Role;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub farm_id: Option<Uuid>,

    // Required profile fields
    pub first_name: String,
    pub last_name: String,
    pub address: String,

    // Optional profile fields
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub farm_id: Option<Uuid>,
    pub role: String,

    pub first_name: String,
    pub last_name: String,
    pub address: String,

    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
            farm_id: row.farm_id,
            role: row.role.parse().unwrap_or(Role::Customer),
            first_name: row.first_name,
            last_name: row.last_name,
            address: row.address,
            phone: row.phone,
            photo_url: row.photo_url,
            date_of_birth: row.date_of_birth,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}