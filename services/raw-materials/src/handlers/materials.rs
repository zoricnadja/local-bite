use axum::{debug_handler, extract::{Path, Query}, response::Response, Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

use common::{
    errors::AppResult,
    middleware::{require_farm, require_role, AuthClaims},
    response::{created, no_content, ok},
};
use crate::dtos::adjust_quantity_request::AdjustQuantityRequest;
use crate::service::service::RawMaterialService;
use crate::dtos::create_raw_material_request::CreateRawMaterialRequest;
use crate::dtos::update_raw_material_request::UpdateRawMaterialRequest;
use crate::models::query::ListQuery;

#[debug_handler]
pub async fn list(
    AuthClaims(claims): AuthClaims,
    Query(q): Query<ListQuery>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;
    let result = _raw_material_service.list(farm_id, &q).await?;
    Ok(ok(result))
}

#[debug_handler]
pub async fn create(
    AuthClaims(claims): AuthClaims,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
    Json(req): Json<CreateRawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let material = _raw_material_service.create(farm_id, req).await?;
    Ok(created(material))
}

#[debug_handler]
pub async fn get_one(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;
    let material = _raw_material_service.get_one(id, farm_id).await?;
    Ok(ok(material))
}

#[debug_handler]
pub async fn update(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
    Json(req): Json<UpdateRawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let updated = _raw_material_service.update(id, farm_id, req).await?;
    Ok(ok(updated))
}

#[debug_handler]
pub async fn delete(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&claims)?;
    _raw_material_service.delete(id, farm_id).await?;
    Ok(no_content())
}

#[debug_handler]
pub async fn low_stock(
    AuthClaims(claims): AuthClaims,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let items = _raw_material_service.low_stock(farm_id).await?;
    Ok(ok(items))
}

#[debug_handler]
pub async fn adjust_quantity(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    Extension(_raw_material_service): Extension<Arc<RawMaterialService>>,
    Json(req): Json<AdjustQuantityRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let updated = _raw_material_service.adjust_quantity(id, farm_id, req).await?;
    Ok(ok(updated))
}