use axum::{
    extract::{Path, Query, State},
    response::Response,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use common::{
    errors::AppResult,
    middleware::{require_farm, require_role, AuthClaims},
    response::{created, no_content, ok},
};

use crate::{
    models::models::{AdjustQuantityRequest, CreateRawMaterialRequest, ListQuery, UpdateRawMaterialRequest},
    service::service::RawMaterialService,
};

pub async fn list(
    AuthClaims(claims): AuthClaims,
    Query(q): Query<ListQuery>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;
    let result = RawMaterialService::list(&pool, farm_id, &q).await?;
    Ok(ok(result))
}

pub async fn create(
    AuthClaims(claims): AuthClaims,
    State(pool): State<PgPool>,
    Json(req): Json<CreateRawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let material = RawMaterialService::create(&pool, farm_id, req).await?;
    Ok(created(material))
}

pub async fn get_one(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;
    let material = RawMaterialService::get_one(&pool, id, farm_id).await?;
    Ok(ok(material))
}

pub async fn update(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(req): Json<UpdateRawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let updated = RawMaterialService::update(&pool, id, farm_id, req).await?;
    Ok(ok(updated))
}

pub async fn delete(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&claims)?;
    RawMaterialService::delete(&pool, id, farm_id).await?;
    Ok(no_content())
}

pub async fn low_stock(
    AuthClaims(claims): AuthClaims,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let items = RawMaterialService::low_stock(&pool, farm_id).await?;
    Ok(ok(items))
}

pub async fn adjust_quantity(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(req): Json<AdjustQuantityRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;
    let updated = RawMaterialService::adjust_quantity(&pool, id, farm_id, req).await?;
    Ok(ok(updated))
}