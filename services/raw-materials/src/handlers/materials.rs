use axum::{
    debug_handler,
    extract::{Path, Query},
    response::Response,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::adjust_quantity_request::AdjustQuantityRequest;
use crate::dtos::create_raw_material_request::CreateRawMaterialRequest;
use crate::dtos::update_raw_material_request::UpdateRawMaterialRequest;
use crate::models::query::ListQuery;
use crate::service::service::RawMaterialService;
use common::{
    errors::AppResult,
    middleware::{require_business, require_role, AuthClaims},
    response::{created, no_content, ok},
};

#[debug_handler]
pub async fn list(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<ListQuery>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let business_id = require_business(&_claims)?;
    let result = _raw_material_service.list(business_id, &_q).await?;
    Ok(ok(result))
}

#[debug_handler]
pub async fn create(
    AuthClaims(_claims): AuthClaims,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
    Json(_req): Json<CreateRawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business_id = require_business(&_claims)?;
    let material = _raw_material_service.create(business_id, _req).await?;
    Ok(created(material))
}

#[debug_handler]
pub async fn get_one(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let business_id = require_business(&_claims)?;
    let material = _raw_material_service.get_one(_id, business_id).await?;
    Ok(ok(material))
}

#[debug_handler]
pub async fn update(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
    Json(_req): Json<UpdateRawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business_id = require_business(&_claims)?;
    let updated = _raw_material_service.update(_id, business_id, _req).await?;
    Ok(ok(updated))
}

#[debug_handler]
pub async fn delete(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER"])?;
    let business_id = require_business(&_claims)?;
    _raw_material_service.delete(_id, business_id).await?;
    Ok(no_content())
}

#[debug_handler]
pub async fn low_stock(
    AuthClaims(_claims): AuthClaims,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    tracing::info!("Low stock service {}", _claims.role);
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business_id = require_business(&_claims)?;
    let items = _raw_material_service.low_stock(business_id).await?;
    Ok(ok(items))
}

#[debug_handler]
pub async fn adjust_quantity(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
    Json(_req): Json<AdjustQuantityRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business_id = require_business(&_claims)?;
    let updated = _raw_material_service
        .adjust_quantity(_id, business_id, _req)
        .await?;
    Ok(ok(updated))
}
