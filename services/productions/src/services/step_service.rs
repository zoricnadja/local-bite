use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::create_process_step_request::CreateProcessStepRequest;
use crate::dtos::production_batch_response::ProductionBatchResponse;
use crate::dtos::update_process_step_request::UpdateProcessStepRequest;
use crate::models::insert_step_params::InsertStepParams;
use crate::models::process_step::ProcessStep;
use crate::models::update_step_params::UpdateStepParams;
use crate::repositories::batch_repository::BatchRepository;
use crate::repositories::step_repository::StepRepository;
use crate::services::batch_service::BatchService;
use common::errors::{AppError, AppResult};

fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

#[derive(Clone)]
pub struct StepService {
    batch_repository: Arc<BatchRepository>,
    batch_service: Arc<BatchService>,
    step_repository: Arc<StepRepository>,
}
impl StepService {
    pub fn new(batch_repository: Arc<BatchRepository>,
        batch_service: Arc<BatchService>,
        step_repository: Arc<StepRepository>
    ) -> Self {
        Self { batch_repository, batch_service, step_repository }
    }


    pub async fn list(&self, batch_id: Uuid, farm_id: Uuid) -> AppResult<ProductionBatchResponse> {
        let batch = self.batch_repository.find_by_id_and_farm(batch_id, farm_id).await?;
        self.batch_service.assemble_detail(batch).await
    }

    pub async fn add(
        &self,
        batch_id: Uuid,
        farm_id: Uuid,
        req: CreateProcessStepRequest,
    ) -> AppResult<ProcessStep> {
        let batch = self.batch_repository.find_by_id_and_farm(batch_id, farm_id).await?;

        if batch.status == "COMPLETED" || batch.status == "CANCELLED" {
            return Err(AppError::BadRequest(
                format!("Cannot add steps to a {} batch", batch.status),
            ));
        }
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("Step name cannot be empty".into()));
        }
        if self.step_repository.order_exists(batch_id, req.step_order, None).await? {
            return Err(AppError::Conflict(format!(
                "A step with order {} already exists in this batch", req.step_order
            )));
        }

        self.step_repository.insert(InsertStepParams {
            id: Uuid::new_v4(),
            batch_id,
            farm_id,
            step_order: req.step_order,
            name: req.name.trim().to_string(),
            description: req.description,
            duration_hours: req.duration_hours.map(dec),
            temperature: req.temperature.map(dec),
        })
            .await
    }

    pub async fn update(
        &self,
        batch_id: Uuid,
        step_id: Uuid,
        farm_id: Uuid,
        req: UpdateProcessStepRequest,
    ) -> AppResult<ProcessStep> {
        self.batch_repository.find_by_id_and_farm(batch_id, farm_id).await?;
        let existing = self.step_repository.find_by_id_and_batch(step_id, batch_id).await?;

        if let Some(new_order) = req.step_order {
            if new_order != existing.step_order
                && self.step_repository.order_exists(batch_id, new_order, Some(step_id)).await?
            {
                return Err(AppError::Conflict(format!(
                    "Step order {} is already taken in this batch", new_order
                )));
            }
        }

        self.step_repository.update(step_id, batch_id, UpdateStepParams {
            step_order: req.step_order.unwrap_or(existing.step_order),
            name: req.name.as_deref().unwrap_or(&existing.name).to_string(),
            description: req.description.as_deref().or(existing.description.as_deref()).map(str::to_string),
            duration_hours: req.duration_hours.map(dec).or(existing.duration_hours),
            temperature: req.temperature.map(dec).or(existing.temperature),
        })
            .await
    }

    pub async fn delete(
        &self,
        batch_id: Uuid,
        step_id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<()> {
        self.batch_repository.find_by_id_and_farm(batch_id, farm_id).await?;

        let rows = self.step_repository.delete(step_id, batch_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!(
                "Step {} not found in batch {}", step_id, batch_id
            )));
        }
        Ok(())
    }
}