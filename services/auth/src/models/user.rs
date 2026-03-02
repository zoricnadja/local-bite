use chrono::NaiveDateTime;
use serde::Serialize;
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
    pub created_at: NaiveDateTime,
}

// Used only for sqlx mapping
#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub farm_id: Option<Uuid>,
    pub role: String,
    pub created_at: NaiveDateTime,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
            farm_id: row.farm_id,
            role: row.role.parse().unwrap_or(Role::Customer),
            created_at: row.created_at,
        }
    }
}