use crate::models::user::{User, UserRow};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use common::errors::AppError;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, u: User) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO users
                (id, email, password_hash, role, farm_id,
                 first_name, last_name, address,
                 phone, photo_url, date_of_birth,
                 created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5,
                 $6, $7, $8,
                 $9, $10, $11,
                 $12, $13)
            "#,
            u.id, u.email, u.password_hash, u.role.as_str(), u.farm_id,
            u.first_name, u.last_name, u.address,
            u.phone, u.photo_url, u.date_of_birth,
            u.created_at, u.updated_at,
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            "SELECT * FROM users WHERE id = $1",
            id
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            "SELECT * FROM users WHERE email = $1",
            email
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let rows = sqlx::query_as!(UserRow, "SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(User::from).collect())
    }

    pub async fn update_user(&self, u: &User) -> Result<User, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            UPDATE users SET
                email          = $1,
                password_hash  = $2,
                role           = $3,
                farm_id        = $4,
                first_name     = $5,
                last_name      = $6,
                address        = $7,
                phone          = $8,
                photo_url      = $9,
                date_of_birth  = $10,
                updated_at     = $11
            WHERE id = $12
            RETURNING *
            "#,
            u.email, u.password_hash, u.role.as_str(), u.farm_id,
            u.first_name, u.last_name, u.address,
            u.phone, u.photo_url, u.date_of_birth,
            u.updated_at, u.id,
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(User::from(row))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_farm_id(
        &self,
        user_id: Uuid,
        farm_id: Option<Uuid>,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE users SET farm_id = $1 WHERE id = $2"#,
            farm_id,
            user_id
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }


    pub async fn clear_farm_id(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE users SET farm_id = NULL, updated_at = now() WHERE id = $1",
            user_id
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}