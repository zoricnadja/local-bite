use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;
use crate::dtos::create_production_batch_request::CreateProductionBatchRequest;
use crate::dtos::process_step_response::ProcessStepResponse;
use crate::dtos::production_batch_response::ProductionBatchResponse;
use crate::dtos::raw_material_response::RawMaterialResponse;
use crate::dtos::update_production_batch_request::UpdateProductionBatchRequest;
use crate::models::batch_raw_material::BatchRawMaterial;
use crate::models::insert_production_params::InsertProductionParams;
use crate::models::insert_raw_material_params::InsertRawMaterialParams;
use crate::models::process_step::ProcessStep;
use crate::models::production_batch::ProductionBatch;
use crate::models::query::ListQuery;
use crate::models::update_production_params::UpdateProductionParams;
use crate::repositories::batch_repository::BatchRepository;
use crate::repositories::raw_materials_repository::RawMaterialsRepository;
use crate::repositories::step_repository::StepRepository;
use crate::services::raw_materials_service::RawMaterialsService;

#[derive(Clone)]
pub struct BatchService{
    batch_repository: Arc<BatchRepository>,
    step_repository: Arc<StepRepository>,
    materials_repository: Arc<RawMaterialsRepository>,
    raw_materials_service: Arc<RawMaterialsService>
}
fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

impl BatchService {
    pub fn new(batch_repository: Arc<BatchRepository>,
                     step_repository: Arc<StepRepository>,
                     materials_repository: Arc<RawMaterialsRepository>,
                     raw_materials_service: Arc<RawMaterialsService>
    ) -> Self {
        Self { batch_repository, step_repository, materials_repository, raw_materials_service }
    }

    // ── List ──────────────────────────────────────────────────────────────────────

    pub async fn list(
        &self,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<PaginatedResponse<ProductionBatch>> {
        let (items, total) = self.batch_repository.list(farm_id, q).await?;
        Ok(PaginatedResponse {
            data: items,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    // ── Create ────────────────────────────────────────────────────────────────────

    pub async fn create(
        &self,
        farm_id: Uuid,
        req: CreateProductionBatchRequest,
        token: &str,
    ) -> AppResult<ProductionBatchResponse> {
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("Batch name cannot be empty".into()));
        }
        if req.process_type.trim().is_empty() {
            return Err(AppError::BadRequest("Process type cannot be empty".into()));
        }
        if let (Some(s), Some(e)) = (req.start_date, req.end_date) {
            if e < s {
                return Err(AppError::BadRequest("end_date cannot be before start_date".into()));
            }
        }

        // Validate all materials before touching the DB
        let mut material_snapshots = Vec::new();
        if let Some(ref materials) = req.raw_materials {
            for m in materials {
                if m.quantity_used <= 0.0 {
                    return Err(AppError::BadRequest(format!(
                        "quantity_used must be > 0 for material {}", m.raw_material_id
                    )));
                }
                let snapshot = self.raw_materials_service.fetch_raw_material(m.raw_material_id, token)
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
                material_snapshots.push((m.clone(), snapshot));
            }
        }

        let batch_id = Uuid::new_v4();
        let batch = self.batch_repository.insert(InsertProductionParams {
            id: batch_id,
            farm_id,
            name: req.name.trim().to_string(),
            process_type: req.process_type.trim().to_string(),
            start_date: req.start_date,
            end_date: req.end_date,
            notes: req.notes,
        })
            .await?;

        for (input, snap) in &material_snapshots {
            self.materials_repository.insert(InsertRawMaterialParams {
                id: Uuid::new_v4(),
                batch_id,
                farm_id,
                raw_material_id: snap.id,
                raw_material_name: snap.name.clone(),
                material_type: snap.material_type.clone(),
                quantity_used: dec(input.quantity_used),
                unit: input.unit.trim().to_string(),
                origin: snap.origin.clone(),
                supplier: snap.supplier.clone(),
            })
                .await?;
        }

        self.assemble_detail(batch).await
    }

    // ── Get one ───────────────────────────────────────────────────────────────────

    pub async fn get_one(&self, id: Uuid, farm_id: Uuid) -> AppResult<ProductionBatchResponse> {
        let batch = self.batch_repository.find_by_id_and_farm(id, farm_id).await?;
        self.assemble_detail(batch).await
    }

    // ── Update ────────────────────────────────────────────────────────────────────

    pub async fn update(
        &self,
        id: Uuid,
        farm_id: Uuid,
        req: UpdateProductionBatchRequest,
    ) -> AppResult<ProductionBatchResponse> {
        let existing = self.batch_repository.find_by_id_and_farm(id, farm_id).await?;

        if let Some(ref new_status) = req.status {
            self.validate_status_transition(&existing.status, new_status)?;
        }

        let new_start = req.start_date.or(existing.start_date);
        let new_end = req.end_date.or(existing.end_date);

        if let (Some(s), Some(e)) = (new_start, new_end) {
            if e < s {
                return Err(AppError::BadRequest("end_date cannot be before start_date".into()));
            }
        }

        let updated = self.batch_repository.update(id, farm_id, UpdateProductionParams {
            name: req.name.as_deref().unwrap_or(&existing.name).to_string(),
            process_type: req.process_type.as_deref().unwrap_or(&existing.process_type).to_string(),
            start_date: new_start,
            end_date: new_end,
            notes: req.notes.as_deref().or(existing.notes.as_deref()).map(str::to_string),
            status: req.status.as_deref().unwrap_or(&existing.status).to_string(),
        })
            .await?;

        self.assemble_detail(updated).await
    }

    // ── Delete ────────────────────────────────────────────────────────────────────

    pub async fn delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let batch = self.batch_repository.find_by_id_and_farm(id, farm_id).await?;

        if batch.status == "IN_PROGRESS" {
            return Err(AppError::BadRequest(
                "Cannot delete a batch that is IN_PROGRESS. Cancel it first.".into(),
            ));
        }

        let rows = self.batch_repository.soft_delete(id, farm_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!("Batch {} not found", id)));
        }
        Ok(())
    }

    // ── Detail assembly (shared with step/material services) ─────────────────────

    pub async fn assemble_detail(&self, batch: ProductionBatch) -> AppResult<ProductionBatchResponse> {
        let (steps, materials) = tokio::try_join!(
            self.step_repository.find_by_batch(batch.id),
            self.materials_repository.find_by_batch(batch.id),
        )?;

        Ok(ProductionBatchResponse {
            id: batch.id,
            farm_id: batch.farm_id,
            name: batch.name,
            process_type: batch.process_type,
            start_date: batch.start_date.map(|d| d.to_string()),
            end_date: batch.end_date.map(|d| d.to_string()),
            status: batch.status,
            notes: batch.notes,
            created_at: batch.created_at,
            updated_at: batch.updated_at,
            steps: steps.into_iter().map(Self::step_to_response).collect(),
            raw_materials: materials.into_iter().map(Self::material_to_response).collect(),
        })
    }

    // ── Status validation ─────────────────────────────────────────────────────────

    pub fn validate_status_transition(&self, current: &str, next: &str) -> AppResult<()> {
        if current == next {
            return Ok(())
        }
        let valid = match current {
            "PLANNED" => matches!(next, "IN_PROGRESS" | "CANCELLED"),
            "IN_PROGRESS" => matches!(next, "COMPLETED"   | "CANCELLED"),
            _ => false,
        };
        if !valid {
            return Err(AppError::BadRequest(format!(
                "Invalid status transition: {} → {}", current, next
            )));
        }
        Ok(())
    }

    // ── Mapping helpers ───────────────────────────────────────────────────────────

    pub fn step_to_response(s: ProcessStep) -> ProcessStepResponse {
        ProcessStepResponse {
            id: s.id,
            step_order: s.step_order,
            name: s.name,
            description: s.description,
            duration_hours: s.duration_hours.map(|v| f64::from_str(&v.to_string()).unwrap_or(0.0)),
            temperature: s.temperature.map(|v| f64::from_str(&v.to_string()).unwrap_or(0.0)),
        }
    }

    pub fn material_to_response(m: BatchRawMaterial) -> RawMaterialResponse {
        RawMaterialResponse {
            id: m.raw_material_id,
            name: m.raw_material_name,
            material_type: m.material_type,
            quantity_used: f64::from_str(&m.quantity_used.to_string()).unwrap_or(0.0),
            unit: m.unit,
            origin: m.origin,
            supplier: m.supplier,
        }
    }
}