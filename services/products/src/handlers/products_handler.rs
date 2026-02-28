use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
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
use crate::dtos::create_product_request::CreateProductRequest;
use crate::dtos::update_product_request::UpdateProductRequest;
use crate::models::query::ListQuery;
use crate::services::{product_service, provenance_service};

// ── GET /products ─────────────────────────────────────────────────────────────

pub async fn list(
    AuthClaims(claims): AuthClaims,
    Query(q): Query<ListQuery>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;

    let result = product_service::list(&pool, farm_id, &q).await?;
    Ok(ok(result))
}

// ── POST /products ────────────────────────────────────────────────────────────

pub async fn create(
    AuthClaims(claims): AuthClaims,
    State(pool): State<PgPool>,
    Json(req): Json<CreateProductRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;

    let product = product_service::create(&pool, farm_id, req).await?;
    Ok(created(product))
}

// ── GET /products/:id ─────────────────────────────────────────────────────────

pub async fn get_one(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;

    let product = product_service::get_one(&pool, id, farm_id).await?;
    Ok(ok(product))
}

// ── GET /products/:id/provenance ──────────────────────────────────────────────

pub async fn provenance(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "CUSTOMER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;

    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or_default();

    let result = provenance_service::get_provenance(&pool, id, farm_id, token).await?;
    Ok(ok(result))
}

// ── PUT /products/:id ─────────────────────────────────────────────────────────

pub async fn update(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(req): Json<UpdateProductRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;

    let updated = product_service::update(&pool, id, farm_id, req).await?;
    Ok(ok(updated))
}

// ── DELETE /products/:id ──────────────────────────────────────────────────────

pub async fn delete(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&claims)?;

    product_service::delete(&pool, id, farm_id).await?;
    Ok(no_content())
}