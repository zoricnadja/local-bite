use axum::{debug_handler, extract::Path, response::IntoResponse, Extension, Json};
use http::StatusCode;
use std::sync::Arc;
use uuid::Uuid;

use common::{
    errors::AppError,
    middleware::AuthClaims,
    response::{created, ok},
};

use crate::dtos::register_request::RegisterRequest;
use crate::dtos::update_business_request::UpdateBusinessRequest;
use crate::dtos::{create_business_request::CreateBusinessRequest};
use crate::service::business_service::BusinessService;

// A short-lived service token exposes only the producer's public name.
pub async fn trace_business(
    Extension(service): Extension<Arc<BusinessService>>,
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    if claims.role != "TRACEABILITY" || claims.business_id != Some(id) {
        return Err(AppError::Forbidden("Traceability scope mismatch".into()));
    }
    let business = service.business_repository.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFound("Business not found".into()))?;
    #[derive(serde::Serialize)]
    struct Producer { name: String }
    Ok(ok(Producer { name: business.name }))
}
// ── POST /businesses ──────────────────────────────────────────────────────────────

pub async fn create_business(
    Extension(business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
    Json(_payload): Json<CreateBusinessRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = business_service.create_business(&_claims, _payload).await?;
    Ok(created(result))
}

// ── POST /businesses/:id/workers ──────────────────────────────────────────────────

pub async fn add_worker(
    Extension(business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_business_id): Path<Uuid>,
    Json(_payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let worker = business_service
        .add_worker(&_claims, _business_id, _payload)
        .await?;
    Ok(ok(worker))
}

// ── GET /businesses/:id ───────────────────────────────────────────────────────────

pub async fn get_business(
    Extension(business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_business_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let business = business_service.get_business(&_claims, _business_id).await?;
    Ok(ok(business))
}

// ── GET /businesses/:id/workers ───────────────────────────────────────────────────

pub async fn list_workers(
    Extension(business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_business_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let workers = business_service.list_workers(&_claims, _business_id).await?;
    Ok(ok(workers))
}

#[debug_handler]
pub async fn list_businesses(
    Extension(_business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
) -> Result<impl IntoResponse, AppError> {
    let businesses = _business_service.list_businesses(&_claims).await?;
    Ok(ok(businesses))
}

#[debug_handler]
pub async fn update_business(
    Extension(_business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_business_id): Path<Uuid>,
    Json(_payload): Json<UpdateBusinessRequest>,
) -> Result<impl IntoResponse, AppError> {
    let business = _business_service
        .update_business(&_claims, _business_id, _payload)
        .await?;
    Ok(ok(business))
}

#[debug_handler]
pub async fn delete_business(
    Extension(_business_service): Extension<Arc<BusinessService>>,
    AuthClaims(_claims): AuthClaims,
    Path(_business_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    _business_service.delete_business(&_claims, _business_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
