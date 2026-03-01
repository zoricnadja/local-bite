use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::insert_step_params::InsertStepParams;
use crate::models::process_step::ProcessStep;
use crate::models::update_step_params::UpdateStepParams;

#[derive(Clone)]
pub struct StepRepository {
    pub pool: PgPool
}

impl StepRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_by_batch(&self, batch_id: Uuid) -> AppResult<Vec<ProcessStep>> {
        Ok(sqlx::query_as!(
            ProcessStep,
            r#"
            SELECT id, batch_id, farm_id, step_order, name, description,
                   duration_hours, temperature, created_at, updated_at
            FROM   process_steps
            WHERE  batch_id = $1
            ORDER  BY step_order ASC
            "#,
            batch_id
        )
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn find_by_id_and_batch(
        &self,
        step_id: Uuid,
        batch_id: Uuid,
    ) -> AppResult<ProcessStep> {
        sqlx::query_as!(
            ProcessStep,
            r#"
            SELECT id, batch_id, farm_id, step_order, name, description,
                   duration_hours, temperature, created_at, updated_at
            FROM   process_steps
            WHERE  id = $1 AND batch_id = $2
            "#,
            step_id, batch_id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Step {} not found", step_id)))
    }

    pub async fn order_exists(
        &self,
        batch_id: Uuid,
        step_order: i32,
        exclude_id: Option<Uuid>,
    ) -> AppResult<bool> {
        Ok(match exclude_id {
            Some(excl) => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM process_steps WHERE batch_id = $1 AND step_order = $2 AND id != $3)",
                batch_id, step_order, excl
            )
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(false),
            None => sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM process_steps WHERE batch_id = $1 AND step_order = $2)",
                batch_id, step_order
            )
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(false),
        })
    }

    pub async fn insert(&self, p: InsertStepParams) -> AppResult<ProcessStep> {
        Ok(sqlx::query_as!(
            ProcessStep,
            r#"
            INSERT INTO process_steps
                (id, batch_id, farm_id, step_order, name, description, duration_hours, temperature)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, batch_id, farm_id, step_order, name, description,
                      duration_hours, temperature, created_at, updated_at
            "#,
            p.id, p.batch_id, p.farm_id, p.step_order,
            p.name, p.description.as_deref(),
            p.duration_hours, p.temperature
        )
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn update(
        &self,
        step_id: Uuid,
        batch_id: Uuid,
        p: UpdateStepParams,
    ) -> AppResult<ProcessStep> {
        Ok(sqlx::query_as!(
            ProcessStep,
            r#"
            UPDATE process_steps SET
                step_order     = $1,
                name           = $2,
                description    = $3,
                duration_hours = $4,
                temperature    = $5
            WHERE id = $6 AND batch_id = $7
            RETURNING id, batch_id, farm_id, step_order, name, description,
                      duration_hours, temperature, created_at, updated_at
            "#,
            p.step_order, p.name, p.description.as_deref(),
            p.duration_hours, p.temperature,
            step_id, batch_id
        )
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn delete(&self, step_id: Uuid, batch_id: Uuid) -> AppResult<u64> {
        Ok(sqlx::query!(
            "DELETE FROM process_steps WHERE id = $1 AND batch_id = $2",
            step_id, batch_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected())
    }
}