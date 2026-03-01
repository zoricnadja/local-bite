use axum::{extract::Path, response::Response, Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::create_process_step_request::CreateProcessStepRequest;
use crate::dtos::update_process_step_request::UpdateProcessStepRequest;
use crate::services::step_service::StepService;
use common::{
    errors::AppResult,
    middleware::{require_farm, require_role, AuthClaims},
    response::{created, no_content, ok},
};
// ── GET /batches/:id/steps ────────────────────────────────────────────────────

pub async fn list_steps(
    AuthClaims(_claims): AuthClaims,
    Path(_batch_id): Path<Uuid>,
    Extension(_step_service): Extension<Arc<StepService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&_claims)?;
    Ok(ok(_step_service.list(_batch_id, farm_id).await?))
}

// ── POST /batches/:id/steps ───────────────────────────────────────────────────

pub async fn add_step(
    AuthClaims(_claims): AuthClaims,
    Path(_batch_id): Path<Uuid>,
    Extension(_step_service): Extension<Arc<StepService>>,
    Json(req): Json<CreateProcessStepRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    Ok(created(_step_service.add(_batch_id, farm_id, req).await?))
}

// ── PUT /batches/:id/steps/:step_id ──────────────────────────────────────────

pub async fn update_step(
    AuthClaims(_claims): AuthClaims,
    Path((_batch_id, step_id)): Path<(Uuid, Uuid)>,
    Extension(_step_service): Extension<Arc<StepService>>,
    Json(req): Json<UpdateProcessStepRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    Ok(ok(_step_service.update(_batch_id, step_id, farm_id, req).await?))
}

// ── DELETE /batches/:id/steps/:step_id ───────────────────────────────────────

pub async fn delete_step(
    AuthClaims(_claims): AuthClaims,
    Path((_batch_id, _step_id)): Path<(Uuid, Uuid)>,
    Extension(_step_service): Extension<Arc<StepService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    _step_service.delete(_batch_id, _step_id, farm_id).await?;
    Ok(no_content())
}