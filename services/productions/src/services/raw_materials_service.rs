use crate::dtos::raw_material_api_data::{RawMaterialApiData, RawMaterialWrapper};
use crate::dtos::raw_material_request::RawMaterialRequest;
use crate::models::batch_raw_material::BatchRawMaterial;
use crate::models::insert_raw_material_params::InsertRawMaterialParams;
use crate::repositories::batch_repository::BatchRepository;
use crate::repositories::raw_materials_repository::RawMaterialsRepository;

use anyhow::{anyhow, Context};
use bigdecimal::BigDecimal;
use common::errors::{AppError, AppResult};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

/// Fetch a single raw material from Raw Materials service to validate it exists
/// and belongs to the same farm, and to snapshot its name/type/origin.

#[derive(Clone)]
pub struct RawMaterialsService {
    batch_repository: Arc<BatchRepository>,
    raw_materials_repository: Arc<RawMaterialsRepository>,
}

fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}
impl RawMaterialsService {
    pub fn new(
        batch_repository: Arc<BatchRepository>,
        raw_materials_repository: Arc<RawMaterialsRepository>,
    ) -> Self {
        Self {
            batch_repository,
            raw_materials_repository,

        }
    }

    pub async fn add(
        &self,
        batch_id: Uuid,
        farm_id: Uuid,
        req: RawMaterialRequest,
        token: &str,
    ) -> AppResult<BatchRawMaterial> {
        let batch = self
            .batch_repository
            .find_by_id_and_farm(batch_id, farm_id)
            .await?;

        if batch.status == "COMPLETED" || batch.status == "CANCELLED" {
            return Err(AppError::BadRequest(format!(
                "Cannot add materials to a {} batch",
                batch.status
            )));
        }
        if !req.quantity_used.is_finite() || req.quantity_used < 0.001 {
            return Err(AppError::BadRequest("quantity_used must be > 0".into()));
        }
        if self
            .raw_materials_repository
            .exists(batch_id, req.raw_material_id)
            .await?
        {
            return Err(AppError::Conflict(
                "This raw material is already linked to the batch. Use the update endpoint instead.".into(),
            ));
        }

        let snap = self
            .fetch_raw_material(req.raw_material_id, token)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        if req.unit.trim() != snap.unit {
            return Err(AppError::BadRequest(format!(
                "Use {} for this material",
                snap.unit
            )));
        }
        let mut tx = self.raw_materials_repository.pool.begin().await?;
        // Lock the batch while changing its materials and checking its status.
        let status: String = sqlx::query_scalar("SELECT status FROM production_batches WHERE id=$1 AND farm_id=$2 AND NOT is_deleted FOR UPDATE")
            .bind(batch_id).bind(farm_id).fetch_one(&mut *tx).await?;
        if status == "COMPLETED" || status == "CANCELLED" {
            return Err(AppError::BadRequest(
                "This batch no longer accepts materials".into(),
            ));
        }
        let operation_id = Uuid::new_v4();
        crate::material_recovery::journal(&self.raw_materials_repository.pool, &mut tx, operation_id, farm_id).await?;
        let material = self
            .raw_materials_repository
            .insert_in(
                &mut tx,
                InsertRawMaterialParams {
                    id: operation_id,
                    batch_id,
                    farm_id,
                    raw_material_id: snap.id,
                    raw_material_name: snap.name,
                    material_type: snap.material_type,
                    quantity_used: dec(req.quantity_used),
                    unit: req.unit.trim().to_string(),
                    origin: snap.origin,
                    supplier: snap.supplier,
                    harvest_date: snap.harvest_date,
                    received_date: snap.received_date,
                    expiry_date: snap.expiry_date,
                },
            )
            .await?;
        let items = serde_json::json!([{ "operation_id": operation_id, "raw_material_id": req.raw_material_id, "quantity": req.quantity_used, "unit": req.unit.trim() }]);
        if let Err(error) = self.consume(&items, token).await {
            self.compensate(&[operation_id], token).await?;
            return Err(error);
        }
        if let Err(error) = tx.commit().await {
            self.compensate(&[operation_id], token).await?;
            return Err(error.into());
        }
        Ok(material)
    }

    pub async fn remove(
        &self,
        batch_id: Uuid,
        raw_material_id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<()> {
        let batch = self
            .batch_repository
            .find_by_id_and_farm(batch_id, farm_id)
            .await?;

        if batch.status == "COMPLETED" {
            return Err(AppError::BadRequest(
                "Cannot remove materials from a COMPLETED batch".into(),
            ));
        }

        let rows = self
            .raw_materials_repository
            .delete(batch_id, raw_material_id, farm_id)
            .await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!(
                "Material {} not found in batch {}",
                raw_material_id, batch_id
            )));
        }
        Ok(())
    }

    pub async fn consume(&self, items: &serde_json::Value, token: &str) -> AppResult<()> {
        self.stock_request("production-consumption", items, token)
            .await
    }

    pub async fn compensate(&self, ids: &[Uuid], token: &str) -> AppResult<()> {
        self.stock_request(
            "production-consumption/release",
            &serde_json::json!(ids),
            token,
        )
        .await
    }

    async fn stock_request(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        token: &str,
    ) -> AppResult<()> {
        let base = std::env::var("RAW_MATERIALS_SERVICE_URL")
            .unwrap_or_else(|_| "http://raw-materials-service:3002".into());
        let secret=std::env::var("JWT_SECRET").map_err(|e|AppError::Internal(e.into()))?;
        let claims=common::jwt::decode_jwt(token,&secret)?.claims;
        let internal=common::service_auth::token("MATERIAL_STOCK",Uuid::nil(),claims.farm_id)?;
        let client = reqwest::Client::new();
        for attempt in 0..3 {
            let result = client
                .post(format!(
                    "{}/internal/{}",
                    base.trim_end_matches('/'),
                    endpoint
                ))
                .bearer_auth(&internal)
                .json(body)
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await;
            match result {
                Ok(response) if response.status().is_success() => return Ok(()),
                Ok(response) if response.status().is_client_error() => {
                    let message = response
                        .json::<serde_json::Value>()
                        .await
                        .unwrap_or_default();
                    return Err(AppError::BadRequest(
                        message
                            .get("error")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unable to update material stock")
                            .into(),
                    ));
                }
                _ if attempt < 2 => continue,
                _ => {
                    return Err(AppError::Internal(anyhow!(
                        "Material stock service unavailable; stock reconciliation may be needed"
                    )))
                }
            }
        }
        unreachable!()
    }

    pub async fn fetch_raw_material(
        &self,
        raw_material_id: Uuid,
        token: &str,
    ) -> anyhow::Result<RawMaterialApiData> {
        let base = std::env::var("RAW_MATERIALS_SERVICE_URL")
            .unwrap_or_else(|_| "http://raw-materials-service:3002".into());
        let url = format!("{}/raw_materials/{}", base, raw_material_id);
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .bearer_auth(token)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .context("Failed to reach raw materials service")?;

        if resp.status().as_u16() == 404 {
            return Err(anyhow!(
                "Raw material {} not found or not owned by your farm",
                raw_material_id
            ));
        }
        if !resp.status().is_success() {
            return Err(anyhow!(
                "Raw materials service returned {} for material {}",
                resp.status(),
                raw_material_id
            ));
        }
        let wrapper: RawMaterialWrapper = resp
            .json()
            .await
            .context("Failed to parse raw material response")?;
        Ok(wrapper.data)
    }
}
