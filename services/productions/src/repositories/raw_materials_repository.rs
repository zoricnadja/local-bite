use sqlx::PgPool;
use uuid::Uuid;

use crate::models::batch_raw_material::BatchRawMaterial;
use crate::models::insert_raw_material_params::InsertRawMaterialParams;
use common::errors::AppResult;

#[derive(Clone)]
pub struct RawMaterialsRepository {
    pub pool: PgPool,
}

impl RawMaterialsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_batch(&self, batch_id: Uuid) -> AppResult<Vec<BatchRawMaterial>> {
        Ok(sqlx::query_as::<_, BatchRawMaterial>("SELECT * FROM batch_raw_materials WHERE batch_id=$1").bind(batch_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn exists(&self, batch_id: Uuid, raw_material_id: Uuid) -> AppResult<bool> {
        Ok(sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM batch_raw_materials WHERE batch_id = $1 AND raw_material_id = $2)",
            batch_id, raw_material_id
        )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(false))
    }

    pub async fn insert_in(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        p: InsertRawMaterialParams,
    ) -> AppResult<BatchRawMaterial> {
        Ok(sqlx::query_as::<_, BatchRawMaterial>(
            "INSERT INTO batch_raw_materials (id,batch_id,business_id,raw_material_id,raw_material_name,material_type,quantity_used,unit,origin,supplier,received_date,expiry_date,harvest_date) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) RETURNING *")
            .bind(p.id).bind(p.batch_id).bind(p.business_id).bind(p.raw_material_id)
            .bind(p.raw_material_name).bind(p.material_type).bind(p.quantity_used)
            .bind(p.unit).bind(p.origin).bind(p.supplier).bind(p.received_date).bind(p.expiry_date).bind(p.harvest_date).fetch_one(&mut **tx).await?)
    }

    pub async fn delete(
        &self,
        batch_id: Uuid,
        raw_material_id: Uuid,
        business_id: Uuid,
    ) -> AppResult<u64> {
        Ok(sqlx::query!(
            "DELETE FROM batch_raw_materials WHERE batch_id = $1 AND raw_material_id = $2 AND business_id = $3",
            batch_id, raw_material_id, business_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected())
    }
}
