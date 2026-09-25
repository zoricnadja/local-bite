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

#[derive(Clone)]
pub struct StepService {
    batch_repository: Arc<BatchRepository>,
    batch_service: Arc<BatchService>,
    step_repository: Arc<StepRepository>,
}
impl StepService {
    pub fn new(
        batch_repository: Arc<BatchRepository>,
        batch_service: Arc<BatchService>,
        step_repository: Arc<StepRepository>,
    ) -> Self {
        Self {
            batch_repository,
            batch_service,
            step_repository,
        }
    }

    pub async fn list(&self, batch_id: Uuid, business_id: Uuid) -> AppResult<ProductionBatchResponse> {
        let batch = self
            .batch_repository
            .find_by_id_and_business(batch_id, business_id)
            .await?;
        self.batch_service.assemble_detail(batch).await
    }

    pub async fn add(
        &self,
        batch_id: Uuid,
        business_id: Uuid,
        mut req: CreateProcessStepRequest,
    ) -> AppResult<ProcessStep> {
        let batch = self
            .batch_repository
            .find_by_id_and_business(batch_id, business_id)
            .await?;

        if batch.status == "COMPLETED" || batch.status == "CANCELLED" {
            return Err(AppError::BadRequest(format!(
                "Cannot add steps to a {} batch",
                batch.status
            )));
        }
        crate::models::process_step::validate_variables(&mut req.variables)?;
        if req.step_order < 1 {
            return Err(AppError::BadRequest("Step order must be positive".into()));
        }
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("Step name cannot be empty".into()));
        }
        if self
            .step_repository
            .order_exists(batch_id, req.step_order, None)
            .await?
        {
            return Err(AppError::Conflict(format!(
                "A step with order {} already exists in this batch",
                req.step_order
            )));
        }

        self.step_repository
            .insert(InsertStepParams {
                id: Uuid::new_v4(),
                batch_id,
                business_id,
                step_order: req.step_order,
                name: req.name.trim().to_string(),
                description: req.description,
                variables: serde_json::to_value(req.variables)
                    .map_err(|e| AppError::BadRequest(e.to_string()))?,
            })
            .await
    }

    pub async fn update(
        &self,
        batch_id: Uuid,
        step_id: Uuid,
        business_id: Uuid,
        mut req: UpdateProcessStepRequest,
    ) -> AppResult<ProcessStep> {
        let batch = self
            .batch_repository
            .find_by_id_and_business(batch_id, business_id)
            .await?;
        if batch.status == "COMPLETED" || batch.status == "CANCELLED" {
            return Err(AppError::BadRequest(
                "Cannot change steps of a closed batch".into(),
            ));
        }
        let existing = self
            .step_repository
            .find_by_id_and_batch(step_id, batch_id)
            .await?;

        if let Some(status) = &req.status {
            if status != &existing.status && !matches!((existing.status.as_str(), status.as_str()),
                ("PLANNED", "IN_PROGRESS") | ("IN_PROGRESS", "COMPLETED")) {
                return Err(AppError::BadRequest("Steps must move from Planned to In progress to Completed".into()));
            }
        }
        if let Some(variables) = &mut req.variables {
            crate::models::process_step::validate_variables(variables)?;
        }
        if req.name.as_ref().is_some_and(|n| n.trim().is_empty())
            || req.step_order.is_some_and(|n| n < 1)
        {
            return Err(AppError::BadRequest(
                "Step name and positive order are required".into(),
            ));
        }
        if let Some(new_order) = req.step_order {
            if new_order != existing.step_order
                && self
                    .step_repository
                    .order_exists(batch_id, new_order, Some(step_id))
                    .await?
            {
                return Err(AppError::Conflict(format!(
                    "Step order {} is already taken in this batch",
                    new_order
                )));
            }
        }

        self.step_repository
            .update(
                step_id,
                batch_id,
                UpdateStepParams {
                    step_order: req.step_order.unwrap_or(existing.step_order),
                    status: req.status.unwrap_or(existing.status),
                    name: req.name.as_deref().unwrap_or(&existing.name).to_string(),
                    description: req
                        .description
                        .as_deref()
                        .or(existing.description.as_deref())
                        .map(str::to_string),
                    variables: match req.variables {
                        Some(v) => serde_json::to_value(v)
                            .map_err(|e| AppError::BadRequest(e.to_string()))?,
                        None => existing.variables,
                    },
                },
            )
            .await
    }

    pub async fn delete(&self, batch_id: Uuid, step_id: Uuid, business_id: Uuid) -> AppResult<()> {
        let batch = self
            .batch_repository
            .find_by_id_and_business(batch_id, business_id)
            .await?;
        if batch.status == "COMPLETED" || batch.status == "CANCELLED" {
            return Err(AppError::BadRequest(
                "Cannot change steps of a closed batch".into(),
            ));
        }

        let rows = self.step_repository.delete(step_id, batch_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!(
                "Step {} not found in batch {}",
                step_id, batch_id
            )));
        }
        Ok(())
    }
}
