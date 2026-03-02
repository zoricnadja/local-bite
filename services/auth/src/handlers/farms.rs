use std::sync::Arc;
use axum::{
    extract::{Path},
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;

use common::{
    errors::AppError,
    middleware::AuthClaims,
    response::{created, ok},
};

use crate::dtos::{add_worker_request::AddWorkerRequest, create_farm_request::CreateFarmRequest};
use crate::service::farm_service::FarmService;

// ── POST /farms ──────────────────────────────────────────────────────────────

pub async fn create_farm(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(claims): AuthClaims,
    Json(payload): Json<CreateFarmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = farm_service.create_farm(&claims, payload).await?;
    Ok(created(result))
}

// ── POST /farms/:id/workers ──────────────────────────────────────────────────

pub async fn add_worker(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(claims): AuthClaims,
    Path(farm_id): Path<Uuid>,
    Json(payload): Json<AddWorkerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let worker = farm_service.add_worker(&claims, farm_id, payload).await?;
    Ok(ok(worker))
}

// ── GET /farms/:id ───────────────────────────────────────────────────────────

pub async fn get_farm(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(claims): AuthClaims,
    Path(farm_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let farm = farm_service.get_farm(&claims, farm_id).await?;
    Ok(ok(farm))
}
