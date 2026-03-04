use std::sync::Arc;
use axum::{debug_handler, extract::{Path}, response::IntoResponse, Extension, Json};
use http::StatusCode;
use uuid::Uuid;

use common::{
    errors::AppError,
    middleware::AuthClaims,
    response::{created, ok},
};

use crate::dtos::{add_worker_request::AddWorkerRequest, create_farm_request::CreateFarmRequest};
use crate::dtos::register_request::RegisterRequest;
use crate::dtos::update_farm_request::UpdateFarmRequest;
use crate::service::farm_service::FarmService;
// ── POST /farms ──────────────────────────────────────────────────────────────

pub async fn create_farm(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
    Json(_payload): Json<CreateFarmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = farm_service.create_farm(&_claims, _payload).await?;
    Ok(created(result))
}

// ── POST /farms/:id/workers ──────────────────────────────────────────────────

pub async fn add_worker(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_farm_id): Path<Uuid>,
    Json(_payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let worker = farm_service.add_worker(&_claims, _farm_id, _payload).await?;
    Ok(ok(worker))
}

// ── GET /farms/:id ───────────────────────────────────────────────────────────

pub async fn get_farm(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_farm_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let farm = farm_service.get_farm(&_claims, _farm_id).await?;
    Ok(ok(farm))
}

// ── GET /farms/:id/workers ───────────────────────────────────────────────────

pub async fn list_workers(
    Extension(farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_farm_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let workers = farm_service.list_workers(&_claims, _farm_id).await?;
    Ok(ok(workers))
}

#[debug_handler]
pub async fn list_farms(
    Extension(_farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
) -> Result<impl IntoResponse, AppError> {
    let farms = _farm_service.list_farms(&_claims).await?;
    Ok(ok(farms))
}

#[debug_handler]
pub async fn update_farm(
    Extension(_farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_farm_id): Path<Uuid>,
    Json(_payload): Json<UpdateFarmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let farm = _farm_service.update_farm(&_claims, _farm_id, _payload).await?;
    Ok(ok(farm))
}

#[debug_handler]
pub async fn delete_farm(
    Extension(_farm_service): Extension<Arc<FarmService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_farm_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    _farm_service.delete_farm(&_claims, _farm_id).await?;
    Ok(StatusCode::NO_CONTENT)
}