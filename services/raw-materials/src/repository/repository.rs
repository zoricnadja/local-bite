use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::models::{ListQuery, RawMaterial};

pub struct RawMaterialRepository;

impl RawMaterialRepository {
    pub async fn find_all(
        pool: &PgPool,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<(Vec<RawMaterial>, i64)> {
        let offset = q.offset();
        let limit = q.limit();
        let type_filter = q.material_type.as_deref().unwrap_or("");
        let search_filter = q.search.as_deref().unwrap_or("");

        let items = sqlx::query_as!(
            RawMaterial,
            r#"
            SELECT id, farm_id, name, material_type, quantity, unit,
                   supplier, origin, harvest_date, expiry_date, notes,
                   low_stock_threshold, is_deleted, created_at, updated_at
            FROM   raw_materials
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR material_type ILIKE $2)
              AND  ($3 = '' OR name ILIKE '%' || $3 || '%')
            ORDER  BY created_at DESC
            LIMIT  $4 OFFSET $5
            "#,
            farm_id, type_filter, search_filter, limit, offset
        )
            .fetch_all(pool)
            .await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM raw_materials
            WHERE  farm_id = $1 AND is_deleted = FALSE
              AND  ($2 = '' OR material_type ILIKE $2)
              AND  ($3 = '' OR name ILIKE '%' || $3 || '%')
            "#,
            farm_id, type_filter, search_filter
        )
            .fetch_one(pool)
            .await?
            .unwrap_or(0);

        Ok((items, total))
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<RawMaterial> {
        sqlx::query_as!(
            RawMaterial,
            r#"
            SELECT id, farm_id, name, material_type, quantity, unit,
                   supplier, origin, harvest_date, expiry_date, notes,
                   low_stock_threshold, is_deleted, created_at, updated_at
            FROM   raw_materials
            WHERE  id = $1 AND farm_id = $2 AND is_deleted = FALSE
            "#,
            id, farm_id
        )
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Raw material {} not found", id)))
    }

    pub async fn insert(
        pool: &PgPool,
        id: Uuid,
        farm_id: Uuid,
        name: &str,
        material_type: &str,
        quantity: BigDecimal,
        unit: &str,
        supplier: Option<&str>,
        origin: Option<&str>,
        harvest_date: Option<chrono::NaiveDate>,
        expiry_date: Option<chrono::NaiveDate>,
        notes: Option<&str>,
        low_stock_threshold: Option<BigDecimal>,
    ) -> AppResult<RawMaterial> {
        let material = sqlx::query_as!(
            RawMaterial,
            r#"
            INSERT INTO raw_materials
                (id, farm_id, name, material_type, quantity, unit,
                 supplier, origin, harvest_date, expiry_date, notes, low_stock_threshold)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
            RETURNING id, farm_id, name, material_type, quantity, unit,
                      supplier, origin, harvest_date, expiry_date, notes,
                      low_stock_threshold, is_deleted, created_at, updated_at
            "#,
            id, farm_id, name, material_type, quantity, unit,
            supplier, origin, harvest_date, expiry_date, notes, low_stock_threshold,
        )
            .fetch_one(pool)
            .await?;

        Ok(material)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        farm_id: Uuid,
        name: &str,
        material_type: &str,
        quantity: BigDecimal,
        unit: &str,
        supplier: Option<&str>,
        origin: Option<&str>,
        harvest_date: Option<chrono::NaiveDate>,
        expiry_date: Option<chrono::NaiveDate>,
        notes: Option<&str>,
        low_stock_threshold: Option<BigDecimal>,
    ) -> AppResult<RawMaterial> {
        let updated = sqlx::query_as!(
            RawMaterial,
            r#"
            UPDATE raw_materials SET
                name                = $1,
                material_type       = $2,
                quantity            = $3,
                unit                = $4,
                supplier            = $5,
                origin              = $6,
                harvest_date        = $7,
                expiry_date         = $8,
                notes               = $9,
                low_stock_threshold = $10
            WHERE id = $11 AND farm_id = $12 AND is_deleted = FALSE
            RETURNING id, farm_id, name, material_type, quantity, unit,
                      supplier, origin, harvest_date, expiry_date, notes,
                      low_stock_threshold, is_deleted, created_at, updated_at
            "#,
            name, material_type, quantity, unit,
            supplier, origin, harvest_date, expiry_date,
            notes, low_stock_threshold, id, farm_id
        )
            .fetch_one(pool)
            .await?;

        Ok(updated)
    }

    pub async fn soft_delete(pool: &PgPool, id: Uuid, farm_id: Uuid) -> AppResult<u64> {
        let rows = sqlx::query!(
            "UPDATE raw_materials SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2 AND is_deleted = FALSE",
            id, farm_id
        )
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows)
    }

    pub async fn find_low_stock(pool: &PgPool, farm_id: Uuid) -> AppResult<Vec<RawMaterial>> {
        let items = sqlx::query_as!(
            RawMaterial,
            r#"
            SELECT id, farm_id, name, material_type, quantity, unit,
                   supplier, origin, harvest_date, expiry_date, notes,
                   low_stock_threshold, is_deleted, created_at, updated_at
            FROM   raw_materials
            WHERE  farm_id             = $1
              AND  is_deleted          = FALSE
              AND  low_stock_threshold IS NOT NULL
              AND  quantity            <= low_stock_threshold
            ORDER  BY (quantity / low_stock_threshold) ASC
            "#,
            farm_id
        )
            .fetch_all(pool)
            .await?;

        Ok(items)
    }

    pub async fn adjust_quantity(
        pool: &PgPool,
        id: Uuid,
        farm_id: Uuid,
        delta: BigDecimal,
    ) -> AppResult<Option<RawMaterial>> {
        let result = sqlx::query_as!(
            RawMaterial,
            r#"
            UPDATE raw_materials
               SET quantity = quantity + $1
            WHERE  id       = $2
              AND  farm_id  = $3
              AND  is_deleted = FALSE
              AND  (quantity + $1) >= 0
            RETURNING id, farm_id, name, material_type, quantity, unit,
                      supplier, origin, harvest_date, expiry_date, notes,
                      low_stock_threshold, is_deleted, created_at, updated_at
            "#,
            delta, id, farm_id
        )
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }
}