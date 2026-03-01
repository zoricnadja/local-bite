use crate::models::user::{User, UserRow};
use sqlx::PgPool;
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

    pub async fn create_user(&self, user: User) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, role, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.role.as_str(),
            user.created_at
        )
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, email, password_hash, role, created_at FROM users WHERE email = $1"#,
            email
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(row.map(User::from))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, email, password_hash, role, created_at FROM users WHERE id = $1"#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(row.map(User::from))
    }
}