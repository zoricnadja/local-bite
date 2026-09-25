use sqlx::PgPool;
use uuid::Uuid;

use crate::models::insert_step_params::InsertStepParams;
use crate::models::process_step::ProcessStep;
use crate::models::update_step_params::UpdateStepParams;
use common::errors::{AppError, AppResult};

#[derive(Clone)]
pub struct StepRepository {
    pub pool: PgPool,
}

impl StepRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_by_batch(&self, batch_id: Uuid) -> AppResult<Vec<ProcessStep>> {
        Ok(sqlx::query_as::<_, ProcessStep>(
            r#"
            SELECT id, batch_id, business_id, step_order, name, description,
                   variables, status, created_at, updated_at
            FROM   process_steps
            WHERE  batch_id = $1
            ORDER  BY step_order ASC
            "#,
        )
        .bind(batch_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn find_by_id_and_batch(
        &self,
        step_id: Uuid,
        batch_id: Uuid,
    ) -> AppResult<ProcessStep> {
        sqlx::query_as::<_, ProcessStep>(
            r#"
            SELECT id, batch_id, business_id, step_order, name, description,
                   variables, status, created_at, updated_at
            FROM   process_steps
            WHERE  id = $1 AND batch_id = $2
            "#,
        )
        .bind(step_id)
        .bind(batch_id)
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
        Ok(sqlx::query_as::<_, ProcessStep>(
            r#"
            INSERT INTO process_steps
                (id, batch_id, business_id, step_order, name, description, variables)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, batch_id, business_id, step_order, name, description,
                      variables, status, created_at, updated_at
            "#,
        )
        .bind(p.id)
        .bind(p.batch_id)
        .bind(p.business_id)
        .bind(p.step_order)
        .bind(p.name)
        .bind(p.description)
        .bind(p.variables)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update(
        &self,
        step_id: Uuid,
        batch_id: Uuid,
        p: UpdateStepParams,
    ) -> AppResult<ProcessStep> {
        Ok(sqlx::query_as::<_, ProcessStep>(
            r#"
            UPDATE process_steps SET
                step_order     = $1,
                name           = $2,
                description    = $3,
                variables      = $4,
                status         = $7
            WHERE id = $5 AND batch_id = $6
            RETURNING id, batch_id, business_id, step_order, name, description,
                      variables, status, created_at, updated_at
            "#,
        )
        .bind(p.step_order)
        .bind(p.name)
        .bind(p.description)
        .bind(p.variables)
        .bind(step_id)
        .bind(batch_id)
        .bind(p.status)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn delete(&self, step_id: Uuid, batch_id: Uuid) -> AppResult<u64> {
        Ok(sqlx::query!(
            "DELETE FROM process_steps WHERE id = $1 AND batch_id = $2",
            step_id,
            batch_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
