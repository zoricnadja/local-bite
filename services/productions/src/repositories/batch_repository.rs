use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::insert_production_params::InsertProductionParams;
use crate::models::production_batch::ProductionBatch;
use crate::models::query::ListQuery;
use crate::models::update_production_params::UpdateProductionParams;

#[derive(Clone)]
pub struct BatchRepository {
    pub pool: PgPool
}

impl BatchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(
        &self,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<(Vec<ProductionBatch>, i64)> {
        let offset = q.offset();
        let limit = q.limit();
        let status_f = q.status.as_deref().unwrap_or("");
        let type_f = q.process_type.as_deref().unwrap_or("");
        let search_f = q.search.as_deref().unwrap_or("");

        let items = sqlx::query_as!(
            ProductionBatch,
            r#"
            SELECT id, farm_id, name, process_type, start_date, end_date,
                   status, notes, is_deleted, created_at, updated_at
            FROM   production_batches
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR status       ILIKE $2)
              AND  ($3 = '' OR process_type ILIKE $3)
              AND  ($4 = '' OR name         ILIKE '%' || $4 || '%')
            ORDER  BY created_at DESC
            LIMIT  $5 OFFSET $6
            "#,
            farm_id, status_f, type_f, search_f, limit, offset
        )
            .fetch_all(&self.pool)
            .await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM production_batches
            WHERE  farm_id = $1 AND is_deleted = FALSE
              AND  ($2 = '' OR status       ILIKE $2)
              AND  ($3 = '' OR process_type ILIKE $3)
              AND  ($4 = '' OR name         ILIKE '%' || $4 || '%')
            "#,
            farm_id, status_f, type_f, search_f
        )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0);

        Ok((items, total))
    }

    pub async fn find_by_id_and_farm(
        &self,
        id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<ProductionBatch> {
        sqlx::query_as!(
            ProductionBatch,
            r#"
            SELECT id, farm_id, name, process_type, start_date, end_date,
                   status, notes, is_deleted, created_at, updated_at
            FROM   production_batches
            WHERE  id = $1 AND farm_id = $2 AND is_deleted = FALSE
            "#,
            id, farm_id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Batch {} not found", id)))
    }



    pub async fn insert(&self, p: InsertProductionParams) -> AppResult<ProductionBatch> {
        Ok(sqlx::query_as!(
            ProductionBatch,
            r#"
            INSERT INTO production_batches
                (id, farm_id, name, process_type, start_date, end_date, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, farm_id, name, process_type, start_date, end_date,
                      status, notes, is_deleted, created_at, updated_at
            "#,
            p.id, p.farm_id, p.name, p.process_type,
            p.start_date, p.end_date, p.notes.as_deref()
        )
            .fetch_one(&self.pool)
            .await?)
    }



    pub async fn update(
        &self,
        id: Uuid,
        farm_id: Uuid,
        p: UpdateProductionParams,
    ) -> AppResult<ProductionBatch> {
        Ok(sqlx::query_as!(
            ProductionBatch,
            r#"
            UPDATE production_batches SET
                name         = $1,
                process_type = $2,
                start_date   = $3,
                end_date     = $4,
                notes        = $5,
                status       = $6
            WHERE id = $7 AND farm_id = $8 AND is_deleted = FALSE
            RETURNING id, farm_id, name, process_type, start_date, end_date,
                      status, notes, is_deleted, created_at, updated_at
            "#,
            p.name, p.process_type, p.start_date, p.end_date,
            p.notes.as_deref(), p.status, id, farm_id
        )
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn soft_delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<u64> {
        Ok(sqlx::query!(
            "UPDATE production_batches SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2",
            id, farm_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected())
    }
}