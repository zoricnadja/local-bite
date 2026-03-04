use crate::models::farms::Farm;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use chrono::Utc;
use common::errors::AppError;

// Workers listing record (lightweight projection)
#[derive(sqlx::FromRow, Clone)]
pub struct WorkerRecord {
    pub id: Uuid,
    pub email: String,
    pub farm_id: Uuid,
}

#[derive(Clone)]
pub struct FarmRepository {
    pub pool: PgPool,
}

impl FarmRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn insert_farm(
        &self,
        f: &Farm,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO farms
                (id, name, owner_id,
                 address, photo_url,
                 phone, description, website,
                 created_at, updated_at)
            VALUES
                ($1, $2, $3,
                 $4, $5,
                 $6, $7, $8,
                 $9, $10)
            "#,
            f.id, f.name, f.owner_id,
            f.address, f.photo_url,
            f.phone, f.description, f.website,
            f.created_at, f.updated_at,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#,
            email
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(false);
        Ok(exists)
    }

    pub async fn insert_worker(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
        farm_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO users (id, email, password_hash, role, farm_id) VALUES ($1, $2, $3, 'WORKER', $4)"#,
            id,
            email,
            password_hash,
            farm_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Farm>, AppError> {
        let row = sqlx::query_as!(
            Farm,
            r#"SELECT * FROM farms WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn find_by_owner(&self, owner_id: Uuid) -> Result<Option<Farm>, AppError> {
        let farm = sqlx::query_as!(
            Farm,
            "SELECT * FROM farms WHERE owner_id = $1",
            owner_id
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(farm)
    }

    pub async fn find_all(&self) -> Result<Vec<Farm>, AppError> {
        let farms = sqlx::query_as!(Farm, "SELECT * FROM farms ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(farms)
    }

    pub async fn update_farm(&self, f: &Farm) -> Result<Farm, AppError> {
        let farm = sqlx::query_as!(
            Farm,
            r#"
            UPDATE farms SET
                name        = $1,
                address     = $2,
                photo_url   = $3,
                phone       = $4,
                description = $5,
                website     = $6,
                updated_at  = $7
            WHERE id = $8
            RETURNING *
            "#,
            f.name, f.address, f.photo_url,
            f.phone, f.description, f.website,
            f.updated_at, f.id,
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(farm)
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    pub async fn delete_farm(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM farms WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_workers_by_farm(&self, farm_id: Uuid) -> Result<Vec<WorkerRecord>, AppError> {
        let rows = sqlx::query_as!(
            WorkerRecord,
            r#"SELECT id, email, farm_id as "farm_id!: Uuid" FROM users WHERE farm_id = $1 AND role = 'WORKER' ORDER BY email"#,
            farm_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
