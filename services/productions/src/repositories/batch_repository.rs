use sqlx::PgPool;
use uuid::Uuid;

use crate::models::insert_production_params::InsertProductionParams;
use crate::models::production_batch::ProductionBatch;
use crate::models::query::ListQuery;
use crate::models::update_production_params::UpdateProductionParams;
use common::errors::{AppError, AppResult};

#[derive(Clone)]
pub struct BatchRepository {
    pub pool: PgPool,
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

        let items = sqlx::query_as::<_, ProductionBatch>(r#"
            SELECT id, farm_id, name, process_type, start_date, end_date,
                   status, notes, is_deleted, created_at, updated_at, output_name, output_type, output_unit, output_quantity, output_expiry_date
            FROM   production_batches
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR status       ILIKE $2)
              AND  ($3 = '' OR process_type ILIKE $3)
              AND  ($4 = '' OR name         ILIKE '%' || $4 || '%')
            ORDER  BY created_at DESC
            LIMIT  $5 OFFSET $6
            "#).bind(farm_id).bind(status_f).bind(type_f).bind(search_f).bind(limit).bind(offset)
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
            farm_id,
            status_f,
            type_f,
            search_f
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok((items, total))
    }

    pub async fn find_by_id_and_farm(&self, id: Uuid, farm_id: Uuid) -> AppResult<ProductionBatch> {
        sqlx::query_as::<_, ProductionBatch>(r#"
            SELECT id, farm_id, name, process_type, start_date, end_date,
                   status, notes, is_deleted, created_at, updated_at, output_name, output_type, output_unit, output_quantity, output_expiry_date
            FROM   production_batches
            WHERE  id = $1 AND farm_id = $2 AND is_deleted = FALSE
            "#).bind(id).bind(farm_id)
.fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Batch {} not found", id)))
    }

    pub async fn insert_in(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        p: InsertProductionParams,
    ) -> AppResult<ProductionBatch> {
        Ok(sqlx::query_as::<_, ProductionBatch>(r#"
            INSERT INTO production_batches
                (id, farm_id, name, process_type, start_date, end_date, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, farm_id, name, process_type, start_date, end_date,
                      status, notes, is_deleted, created_at, updated_at, output_name, output_type, output_unit, output_quantity, output_expiry_date
            "#).bind(p.id).bind(p.farm_id).bind(p.name).bind(p.process_type).bind(p.start_date).bind(p.end_date).bind(p.notes.as_deref())
.fetch_one(&mut **tx)
        .await?)
    }

    pub async fn update(
        &self,
        id: Uuid,
        farm_id: Uuid,
        p: UpdateProductionParams,
        expected_status: &str,
    ) -> AppResult<ProductionBatch> {
        Ok(sqlx::query_as::<_, ProductionBatch>(r#"
            UPDATE production_batches SET
                name         = $1,
                process_type = $2,
                start_date   = $3,
                end_date     = $4,
                notes        = $5,
                status       = $6, output_name=$9, output_type=$10, output_unit=$11, output_quantity=$12, output_expiry_date=$13
            WHERE id = $7 AND farm_id = $8 AND is_deleted = FALSE AND status=$14
            RETURNING id, farm_id, name, process_type, start_date, end_date,
                      status, notes, is_deleted, created_at, updated_at, output_name, output_type, output_unit, output_quantity, output_expiry_date
            "#).bind(p.name).bind(p.process_type).bind(p.start_date).bind(p.end_date).bind(p.notes.as_deref()).bind(p.status).bind(id).bind(farm_id).bind(p.output_name).bind(p.output_type).bind(p.output_unit).bind(p.output_quantity).bind(p.output_expiry_date).bind(expected_status)
.fetch_one(&self.pool)
        .await?)
    }

    pub async fn soft_delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<u64> {
        Ok(sqlx::query!(
            "UPDATE production_batches SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2",
            id,
            farm_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
