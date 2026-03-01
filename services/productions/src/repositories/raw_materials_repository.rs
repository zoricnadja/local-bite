use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppResult;
use crate::models::batch_raw_material::BatchRawMaterial;
use crate::models::insert_raw_material_params::InsertRawMaterialParams;

#[derive(Clone)]
pub struct RawMaterialsRepository {
    pub pool: PgPool
}

impl RawMaterialsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_batch(&self, batch_id: Uuid) -> AppResult<Vec<BatchRawMaterial>> {
        Ok(sqlx::query_as!(
            BatchRawMaterial,
            r#"
            SELECT id, batch_id, farm_id, raw_material_id, raw_material_name,
                   material_type, quantity_used, unit, origin, supplier
            FROM   batch_raw_materials
            WHERE  batch_id = $1
            "#,
            batch_id
        )
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn exists(
        &self,
        batch_id: Uuid,
        raw_material_id: Uuid,
    ) -> AppResult<bool> {
        Ok(sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM batch_raw_materials WHERE batch_id = $1 AND raw_material_id = $2)",
            batch_id, raw_material_id
        )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(false))
    }

    pub async fn insert(&self, p: InsertRawMaterialParams) -> AppResult<BatchRawMaterial> {
        Ok(sqlx::query_as!(
            BatchRawMaterial,
            r#"
            INSERT INTO batch_raw_materials
                (id, batch_id, farm_id, raw_material_id, raw_material_name,
                 material_type, quantity_used, unit, origin, supplier)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            ON CONFLICT (batch_id, raw_material_id) DO NOTHING
            RETURNING id, batch_id, farm_id, raw_material_id, raw_material_name,
                      material_type, quantity_used, unit, origin, supplier
            "#,
            p.id, p.batch_id, p.farm_id,
            p.raw_material_id, p.raw_material_name, p.material_type,
            p.quantity_used, p.unit,
            p.origin.as_deref(), p.supplier.as_deref()
        )
            .fetch_one(&self.pool)
            .await?)
    }

    pub async fn delete(
        &self,
        batch_id: Uuid,
        raw_material_id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<u64> {
        Ok(sqlx::query!(
            "DELETE FROM batch_raw_materials WHERE batch_id = $1 AND raw_material_id = $2 AND farm_id = $3",
            batch_id, raw_material_id, farm_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected())
    }
}