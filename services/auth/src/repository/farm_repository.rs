use crate::models::farms::Farm;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use chrono::Utc;
use common::errors::AppError;

#[derive(Clone)]
pub struct FarmRepository {
    pub pool: PgPool,
}

impl FarmRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn insert_farm_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        name: &str,
        owner_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO farms (id, name, owner_id) VALUES ($1, $2, $3)"#,
            id,
            name,
            owner_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn set_user_farm_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        farm_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE users SET farm_id = $1 WHERE id = $2"#,
            farm_id,
            user_id
        )
        .execute(&mut **tx)
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

    pub fn build_farm(&self, id: Uuid, name: String, owner_id: Uuid) -> Farm {
        Farm {
            id,
            name,
            owner_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}
