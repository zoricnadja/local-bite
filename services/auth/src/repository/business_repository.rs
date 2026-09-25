use crate::models::businesses::Business;

use common::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

// Workers listing record (lightweight projection)
#[derive(sqlx::FromRow, Clone)]
pub struct WorkerRecord {
    pub id: Uuid,
    pub email: String,
    pub business_id: Uuid,
}

#[derive(Clone)]
pub struct BusinessRepository {
    pub pool: PgPool,
}

impl BusinessRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_business(&self, f: &Business) -> Result<(), AppError> {
        let mut tx=self.pool.begin().await?;
        sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE").bind(f.owner_id).fetch_one(&mut *tx).await?;
        let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM businesses WHERE owner_id=$1)").bind(f.owner_id).fetch_one(&mut *tx).await?;
        if exists { return Err(AppError::Conflict("You already have a business".into())); }
        sqlx::query("INSERT INTO businesses(id,name,owner_id,address,phone,description,website,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)")
            .bind(f.id).bind(&f.name).bind(f.owner_id).bind(&f.address).bind(&f.phone).bind(&f.description).bind(&f.website).bind(f.created_at).bind(f.updated_at).execute(&mut *tx).await?;
        sqlx::query("UPDATE users SET business_id=$1 WHERE id=$2").bind(f.id).bind(f.owner_id).execute(&mut *tx).await?;
        tx.commit().await?; Ok(())
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

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Business>, AppError> {
        let row = sqlx::query_as!(Business, r#"SELECT * FROM businesses WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn find_by_owner(&self, owner_id: Uuid) -> Result<Option<Business>, AppError> {
        let business = sqlx::query_as!(Business, "SELECT * FROM businesses WHERE owner_id = $1", owner_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(business)
    }

    pub async fn find_all(&self) -> Result<Vec<Business>, AppError> {
        let businesses = sqlx::query_as!(Business, "SELECT * FROM businesses ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(businesses)
    }

    pub async fn update_business(&self, f: &Business) -> Result<Business, AppError> {
        let business = sqlx::query_as!(
            Business,
            r#"
            UPDATE businesses SET
                name        = $1,
                address     = $2,
                phone       = $3,
                description = $4,
                website     = $5,
                updated_at  = $6
            WHERE id = $7
            RETURNING *
            "#,
            f.name,
            f.address,
            f.phone,
            f.description,
            f.website,
            f.updated_at,
            f.id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(business)
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    pub async fn delete_business(&self, id: Uuid) -> Result<(), AppError> {
        let mut tx=self.pool.begin().await?;
        sqlx::query("UPDATE users SET business_id=NULL WHERE business_id=$1").bind(id).execute(&mut *tx).await?;
        sqlx::query("DELETE FROM businesses WHERE id=$1").bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn list_workers_by_business(&self, business_id: Uuid) -> Result<Vec<WorkerRecord>, AppError> {
        let rows = sqlx::query_as!(
            WorkerRecord,
            r#"SELECT id, email, business_id as "business_id!: Uuid" FROM users WHERE business_id = $1 AND role = 'WORKER' ORDER BY email"#,
            business_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
