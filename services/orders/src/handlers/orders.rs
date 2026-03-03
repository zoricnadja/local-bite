use axum::{debug_handler, extract::{Path, Query}, http::HeaderMap, response::Response, Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::analytics::analytics_query::AnalyticsQuery;
use crate::dtos::order::create_order_request::CreateOrderRequest;
use crate::dtos::order::list_orders_query::ListOrdersQuery;
use crate::dtos::order::update_status_request::UpdateStatusRequest;
use crate::services::order_service::OrderService;
use common::{
    errors::AppResult,
    middleware::{require_farm, require_role, AuthClaims},
    response::{created, no_content, ok},
};
// ── GET /orders?page=1&status=PENDING&search=Petar ───────────────────────────

#[debug_handler]
pub async fn list(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<ListOrdersQuery>,
    Extension(_order_service): Extension<Arc<OrderService>>
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&_claims)?;

    let result = _order_service
        .list_orders(farm_id, &_q)
        .await?;

    Ok(ok(result))
}

// ── GET /orders/user/{id}?page=1&status=PENDING&search=Petar ───────────────────────────
#[debug_handler]
pub async fn get_orders_by_user(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<ListOrdersQuery>,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>
) -> AppResult<Response> {
    require_role(&_claims, &["CUSTOMER"])?;

    let result = _order_service
        .find_all_by_user_id(_id, &_q)
        .await?;

    Ok(ok(result))
}

// ── POST /orders ──────────────────────────────────────────────────────────────

#[debug_handler]
pub async fn create(
    AuthClaims(_claims): AuthClaims,
    _headers: HeaderMap,
    Extension(_order_service): Extension<Arc<OrderService>>,
    Json(_req): Json<CreateOrderRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER", "CUSTOMER"])?;
    let token = extract_token(&_headers);

    let result = _order_service
        .create_order(_claims.sub, &*_claims.email, _req, &token)
        .await?;

    Ok(created(result))
}

// ── GET /orders/:id ───────────────────────────────────────────────────────────

#[debug_handler]
pub async fn get_one(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER", "CUSTOMER", "SYSTEM_ADMIN"])?;

    let result = _order_service
        .get_order(_id)
        .await?;

    Ok(ok(result))
}

// ── PUT /orders/:id/status ────────────────────────────────────────────────────

#[debug_handler]
pub async fn update_status(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>,
    Json(_req): Json<UpdateStatusRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;

    let result = _order_service
        .update_status(_id, farm_id, _req, &_claims.role)
        .await?;

    Ok(ok(result))
}

// ── DELETE /orders/:id ────────────────────────────────────────────────────────

#[debug_handler]
pub async fn delete(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&_claims)?;

    _order_service
        .delete_order(_id, farm_id)
        .await?;

    Ok(no_content())
}

// ── GET /orders/analytics?from=2024-01-01&to=2024-12-31 ──────────────────────

#[debug_handler]
pub async fn analytics(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<AnalyticsQuery>,
    Extension(_order_service): Extension<Arc<OrderService>>
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&_claims)?;

    let from = _q.from.as_deref().unwrap_or("");
    let to   = _q.to.as_deref().unwrap_or("");

    let result = _order_service
        .get_analytics(farm_id, from, to)
        .await?;

    Ok(ok(result))
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn extract_token(headers: &HeaderMap) -> String {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or_default()
        .to_string()
}
