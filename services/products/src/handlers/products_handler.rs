use axum::{
    debug_handler,
    extract::{Path, Query},
    http::HeaderMap,
    response::Response,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::create_product_request::CreateProductRequest;

use crate::dtos::update_product_request::UpdateProductRequest;
use crate::models::query::ListQuery;
use crate::services::product_service::ProductService;
use crate::services::provenance_service::ProvenanceService;
use common::{
    errors::AppResult,
    middleware::{require_business, require_role, AuthClaims},
    response::{created, no_content, ok},
};
// ── GET /products/business ─────────────────────────────────────────────────────────────

#[debug_handler]
pub async fn list_by_business(
    AuthClaims(_claims): AuthClaims,
    Query(mut _q): Query<ListQuery>,
    Extension(_product_service): Extension<Arc<ProductService>>,
) -> AppResult<Response> {
    require_role(
        &_claims,
        &["BUSINESS_OWNER", "WORKER", "SYSTEM_ADMIN", "CUSTOMER"],
    )?;
    let business_id = require_business(&_claims)?;

    let result = _product_service.find_all_by_business_id(business_id, &_q).await?;
    Ok(ok(result))
}

// ── GET /products ─────────────────────────────────────────────────────────────

#[debug_handler]
pub async fn list(
    AuthClaims(_claims): AuthClaims,
    Query(mut _q): Query<ListQuery>,
    Extension(_product_service): Extension<Arc<ProductService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["SYSTEM_ADMIN", "CUSTOMER"])?;

    if _claims.role == "CUSTOMER" { _q.is_active=Some(true); }
    let result = _product_service.find_all(&_q).await?;
    Ok(ok(result))
}

// ── POST /products ────────────────────────────────────────────────────────────

#[debug_handler]
pub async fn create(
    AuthClaims(_claims): AuthClaims,
    Extension(_product_service): Extension<Arc<ProductService>>,
    Json(_req): Json<CreateProductRequest>,
) -> AppResult<Response> {
    require_role(
        &_claims,
        &["BUSINESS_OWNER", "WORKER"],
    )?;
    let business_id = require_business(&_claims)?;

    let product = _product_service.create(business_id, _req).await?;
    Ok(created(product))
}

// ── GET /products/:id ─────────────────────────────────────────────────────────

#[debug_handler]
pub async fn get_one(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_product_service): Extension<Arc<ProductService>>,
) -> AppResult<Response> {
    require_role(
        &_claims,
        &["BUSINESS_OWNER", "WORKER", "SYSTEM_ADMIN", "CUSTOMER"],
    )?;

    let product = _product_service.get_one(_id).await?;
    crate::services::product_service::authorize_read(&product, &_claims)?;
    Ok(ok(product))
}

// ── GET /products/:id/provenance ──────────────────────────────────────────────

#[debug_handler]
pub async fn provenance(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    _headers: HeaderMap,
    Extension(product_service): Extension<Arc<ProductService>>,
    Extension(_provenance_service): Extension<Arc<ProvenanceService>>,
) -> AppResult<Response> {
    require_role(
        &_claims,
        &["BUSINESS_OWNER", "WORKER", "CUSTOMER", "SYSTEM_ADMIN"],
    )?;
    let product = product_service.get_one(_id).await?;
    crate::services::product_service::authorize_read(&product, &_claims)?;
    let business_id = product.business_id;

    let token = _headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or_default();

    let result = _provenance_service
        .get_provenance(_id, business_id, token)
        .await?;
    Ok(ok(result))
}

// ── PUT /products/:id ─────────────────────────────────────────────────────────

#[debug_handler]
pub async fn update(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_product_service): Extension<Arc<ProductService>>,
    Json(req): Json<UpdateProductRequest>,
) -> AppResult<Response> {
    require_role(
        &_claims,
        &["BUSINESS_OWNER", "WORKER"],
    )?;
    let business_id = require_business(&_claims)?;

    let updated = _product_service.update(_id, business_id, req).await?;
    Ok(ok(updated))
}

// ── DELETE /products/:id ──────────────────────────────────────────────────────

#[debug_handler]
pub async fn delete(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_product_service): Extension<Arc<ProductService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER"])?;
    let business_id = require_business(&_claims)?;

    _product_service.delete(_id, business_id).await?;
    Ok(no_content())
}
