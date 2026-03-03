use std::str::FromStr;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use bigdecimal::BigDecimal;
use uuid::Uuid;
use common::errors::{AppError, AppResult};
use crate::dtos::raw_material_api_data::{RawMaterialApiData, RawMaterialWrapper};
use crate::dtos::raw_material_request::RawMaterialRequest;
use crate::models::batch_raw_material::BatchRawMaterial;
use crate::models::insert_raw_material_params::InsertRawMaterialParams;
use crate::repositories::batch_repository::BatchRepository;
use crate::repositories::raw_materials_repository::RawMaterialsRepository;
use crate::repositories::step_repository::StepRepository;

/// Fetch a single raw material from Raw Materials service to validate it exists
/// and belongs to the same farm, and to snapshot its name/type/origin.

#[derive(Clone)]
pub struct RawMaterialsService {
    batch_repository: Arc<BatchRepository>,
    raw_materials_repository: Arc<RawMaterialsRepository>,
    step_repository: Arc<StepRepository>
}

fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}
impl RawMaterialsService {
    pub fn new(batch_repository: Arc<BatchRepository>,
               raw_materials_repository: Arc<RawMaterialsRepository>,
               step_repository: Arc<StepRepository>
    ) -> Self {
        Self { batch_repository, raw_materials_repository, step_repository }
    }

    pub async fn add(
        &self,
        batch_id: Uuid,
        farm_id: Uuid,
        req: RawMaterialRequest,
        token: &str,
    ) -> AppResult<BatchRawMaterial> {
        let batch = self.batch_repository.find_by_id_and_farm(batch_id, farm_id).await?;

        if batch.status == "COMPLETED" || batch.status == "CANCELLED" {
            return Err(AppError::BadRequest(
                format!("Cannot add materials to a {} batch", batch.status),
            ));
        }
        if req.quantity_used <= 0.0 {
            return Err(AppError::BadRequest("quantity_used must be > 0".into()));
        }
        if self.raw_materials_repository.exists(batch_id, req.raw_material_id).await? {
            return Err(AppError::Conflict(
                "This raw material is already linked to the batch. Use the update endpoint instead.".into(),
            ));
        }

        let snap = self.fetch_raw_material(req.raw_material_id, token)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        self.raw_materials_repository.insert(InsertRawMaterialParams {
            id: Uuid::new_v4(),
            batch_id,
            farm_id,
            raw_material_id:   snap.id,
            raw_material_name: snap.name,
            material_type:     snap.material_type,
            quantity_used:     dec(req.quantity_used),
            unit:              req.unit.trim().to_string(),
            origin:            snap.origin,
            supplier:          snap.supplier,
        })
            .await
    }

    pub async fn remove(
        &self,
        batch_id: Uuid,
        raw_material_id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<()> {
        let batch = self.batch_repository.find_by_id_and_farm(batch_id, farm_id).await?;

        if batch.status == "COMPLETED" {
            return Err(AppError::BadRequest(
                "Cannot remove materials from a COMPLETED batch".into(),
            ));
        }

        let rows = self.raw_materials_repository.delete(batch_id, raw_material_id, farm_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!(
                "Material {} not found in batch {}", raw_material_id, batch_id
            )));
        }
        Ok(())
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
            return Err(anyhow!("Raw material {} not found or not owned by your farm", raw_material_id));
        }
        if !resp.status().is_success() {
            return Err(anyhow!(
                "Raw materials service returned {} for material {}",
                resp.status(), raw_material_id
            ));
        }
        let wrapper: RawMaterialWrapper = resp
            .json()
            .await
            .context("Failed to parse raw material response")?;
        Ok(wrapper.data)
    }
}