use axum::{extract::{Path, Query}, http::HeaderMap, response::Response, Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::create_production_batch_request::CreateProductionBatchRequest;
use crate::dtos::update_production_batch_request::UpdateProductionBatchRequest;
use crate::models::query::ListQuery;
use crate::services::batch_service::BatchService;
use common::{
    errors::AppResult,
    middleware::{require_farm, require_role, AuthClaims},
    response::{created, no_content, ok},
};
// ── GET /batches ──────────────────────────────────────────────────────────────

pub async fn list(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<ListQuery>,
    Extension(_batch_service): Extension<Arc<BatchService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    Ok(ok(_batch_service.list(farm_id, &_q).await?))
}

// ── POST /batches ─────────────────────────────────────────────────────────────

pub async fn create(
    AuthClaims(_claims): AuthClaims,
    headers: HeaderMap,
    Extension(_batch_service): Extension<Arc<BatchService>>,
    Json(_req): Json<CreateProductionBatchRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    let token = extract_token(&headers);
    Ok(created(_batch_service.create(farm_id, _req, &token).await?))
}

// ── GET /batches/:id ──────────────────────────────────────────────────────────

pub async fn get_one(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_batch_service): Extension<Arc<BatchService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    Ok(ok(_batch_service.get_one(_id, farm_id).await?))
}

// ── PUT /batches/:id ──────────────────────────────────────────────────────────

pub async fn update(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_batch_service): Extension<Arc<BatchService>>,
    Json(_req): Json<UpdateProductionBatchRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;
    Ok(ok(_batch_service.update(_id, farm_id, _req).await?))
}

// ── DELETE /batches/:id ───────────────────────────────────────────────────────

pub async fn delete(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_batch_service): Extension<Arc<BatchService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&_claims)?;
    _batch_service.delete(_id, farm_id).await?;
    Ok(no_content())
}

// ── Shared token extractor ────────────────────────────────────────────────────

pub fn extract_token(headers: &HeaderMap) -> String {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or_default()
        .to_string()
}