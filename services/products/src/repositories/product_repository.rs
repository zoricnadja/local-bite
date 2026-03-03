use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::insert_product_params::InsertParams;
use crate::models::product::Product;
use crate::models::query::ListQuery;
use crate::models::update_product_params::UpdateParams;

#[derive(Clone)]
pub struct ProductRepository {
    pub pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    // ── List ──────────────────────────────────────────────────────────────────────

    pub async fn list(
        &self,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<(Vec<Product>, i64)> {
        let offset = q.offset();
        let limit = q.limit();
        let type_filter = q.product_type.as_deref().unwrap_or("");
        let search = q.search.as_deref().unwrap_or("");
        let active_only = q.active_only.unwrap_or(false);

        let items = sqlx::query_as!(
            Product,
            r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR product_type ILIKE $2)
              AND  ($3 = '' OR name ILIKE '%' || $3 || '%')
              AND  (NOT $4    OR is_active = TRUE)
            ORDER  BY created_at DESC
            LIMIT  $5 OFFSET $6
            "#,
            farm_id, type_filter, search, active_only, limit, offset
        )
            .fetch_all(&self.pool)
            .await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM products
            WHERE  farm_id = $1 AND is_deleted = FALSE
              AND  ($2 = '' OR product_type ILIKE $2)
              AND  ($3 = '' OR name ILIKE '%' || $3 || '%')
              AND  (NOT $4    OR is_active = TRUE)
            "#,
            farm_id, type_filter, search, active_only
        )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0);

        Ok((items, total))
    }

    // ── Find ──────────────────────────────────────────────────────────────────────

    pub async fn find_by_id_and_farm(&self, id: Uuid, farm_id: Uuid) -> AppResult<Product> {
        sqlx::query_as!(
            Product,
            r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  id = $1 AND farm_id = $2 AND is_deleted = FALSE
            "#,
            id, farm_id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))
    }
    
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Product> {
        sqlx::query_as!(
            Product,
            r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  id = $1 AND is_deleted = FALSE
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))
    }

    pub async fn find_by_qr_token(&self, qr_token: Uuid) -> AppResult<Product> {
        sqlx::query_as!(
            Product,
            r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  qr_token  = $1
              AND  is_active = TRUE
              AND  is_deleted = FALSE
            "#,
            qr_token
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found or no longer active".into()))
    }

    // ── Insert ────────────────────────────────────────────────────────────────────

    pub async fn insert(&self, p: InsertParams) -> AppResult<Product> {
        Ok(sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products
                (id, farm_id, name, product_type, description, quantity, unit,
                 price, batch_id, qr_token, qr_path)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                      batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                      created_at, updated_at
            "#,
            p.id, p.farm_id, p.name, p.product_type, p.description,
            p.quantity, p.unit, p.price, p.batch_id, p.qr_token, p.qr_path,
        )
            .fetch_one(&self.pool)
            .await?)
    }

    // ── Update ────────────────────────────────────────────────────────────────────
    pub async fn update(
        &self,
        id: Uuid,
        farm_id: Uuid,
        p: UpdateParams,
    ) -> AppResult<Product> {
        Ok(sqlx::query_as!(
            Product,
            r#"
            UPDATE products SET
                name         = $1,
                product_type = $2,
                description  = $3,
                quantity     = $4,
                unit         = $5,
                price        = $6,
                batch_id     = $7,
                is_active    = $8
            WHERE id = $9 AND farm_id = $10 AND is_deleted = FALSE
            RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                      batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                      created_at, updated_at
            "#,
            p.name, p.product_type, p.description, p.quantity,
            p.unit, p.price, p.batch_id, p.is_active,
            id, farm_id
        )
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn soft_delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<u64> {
        Ok(sqlx::query!(
            "UPDATE products SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2 AND is_deleted = FALSE",
            id, farm_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected())
    }

    // ── QR / Image ────────────────────────────────────────────────────────────────

    pub async fn set_qr_path(&self, id: Uuid, qr_path: &str) -> AppResult<()> {
        sqlx::query!("UPDATE products SET qr_path = $1 WHERE id = $2", qr_path, id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_qr_path_returning(
        &self,
        id: Uuid,
        farm_id: Uuid,
        qr_path: &str,
    ) -> AppResult<Product> {
        Ok(sqlx::query_as!(
            Product,
            r#"
            UPDATE products SET qr_path = $1 WHERE id = $2 AND farm_id = $3
            RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                      batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                      created_at, updated_at
            "#,
            qr_path, id, farm_id
        )
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn set_image_path(
        &self,
        id: Uuid,
        farm_id: Uuid,
        image_path: &str,
    ) -> AppResult<Product> {
        Ok(sqlx::query_as!(
            Product,
            r#"
            UPDATE products SET image_path = $1
            WHERE id = $2 AND farm_id = $3
            RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                      batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                      created_at, updated_at
            "#,
            image_path, id, farm_id
        )
            .fetch_one(&self.pool)
            .await?)
    }
}