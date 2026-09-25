use axum::{
    debug_handler,
    extract::{Path, Query},
    http::HeaderMap,
    response::Response,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::analytics::analytics_query::AnalyticsQuery;
use crate::dtos::order::create_order_request::CreateOrderRequest;
use crate::dtos::order::list_orders_query::ListOrdersQuery;
use crate::dtos::order::update_status_request::UpdateStatusRequest;
use crate::services::order_service::OrderService;
use common::{
    errors::AppResult,
    middleware::{require_business, require_role, AuthClaims},
    response::{created, no_content, ok},
};
// ── GET /orders?page=1&status=PENDING&search=Petar ───────────────────────────

#[debug_handler]
pub async fn list(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<ListOrdersQuery>,
    Extension(_order_service): Extension<Arc<OrderService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let business_id = require_business(&_claims)?;

    let result = _order_service.list_orders(business_id, &_q).await?;

    Ok(ok(result))
}

// ── GET /orders/user/{id}?page=1&status=PENDING&search=Petar ───────────────────────────
#[debug_handler]
pub async fn get_orders_by_user(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<ListOrdersQuery>,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["CUSTOMER"])?;

    if _id != _claims.sub { return Err(common::errors::AppError::Forbidden("Only your own orders are available".into())); }
    let result = _order_service.find_all_by_user_id(_claims.sub, &_q).await?;

    Ok(ok(result))
}

// ── POST /orders ──────────────────────────────────────────────────────────────

#[debug_handler]
pub async fn create(
    AuthClaims(_claims): AuthClaims,
    _headers: HeaderMap,
    Extension(_order_service): Extension<Arc<OrderService>>,
    Json(mut _req): Json<CreateOrderRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["CUSTOMER"])?;
    let token = extract_token(&_headers);
    let key = _headers.get("idempotency-key").and_then(|v|v.to_str().ok()).and_then(|v|v.parse::<Uuid>().ok())
        .ok_or_else(||common::errors::AppError::BadRequest("A UUID Idempotency-Key header is required".into()))?;
    _req.customer_id=Some(_claims.sub);

    let base=std::env::var("AUTH_SERVICE_URL").unwrap_or_else(|_| "http://auth-service:3001".into());
    let profile:serde_json::Value=reqwest::Client::new().get(format!("{}/me",base)).bearer_auth(&token).timeout(std::time::Duration::from_secs(5)).send().await.map_err(|e| common::errors::AppError::Internal(e.into()))?.error_for_status().map_err(|e| common::errors::AppError::Internal(e.into()))?.json().await.map_err(|e| common::errors::AppError::Internal(e.into()))?;
    _req.customer_name=Some(format!("{} {}",profile["first_name"].as_str().unwrap_or(""),profile["last_name"].as_str().unwrap_or("")).trim().to_owned());
    let email=profile["email"].as_str().ok_or_else(|| common::errors::AppError::BadRequest("Account email unavailable".into()))?;
    _req.customer_email=Some(email.to_owned());
    let result = _order_service
        .create_order(_claims.sub, email, _req, &token, key)
        .await?;

    Ok(created(result))
}

// ── GET /orders/:id ───────────────────────────────────────────────────────────

#[debug_handler]
pub async fn get_one(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>,
) -> AppResult<Response> {
    require_role(
        &_claims,
        &["BUSINESS_OWNER", "WORKER", "CUSTOMER", "SYSTEM_ADMIN"],
    )?;

    let result = _order_service.get_order(_id, &_claims).await?;

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
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let business_id = require_business(&_claims)?;

    let result = _order_service
        .update_status(_id, business_id, _req, &_claims.role)
        .await?;

    Ok(ok(result))
}

// ── DELETE /orders/:id ────────────────────────────────────────────────────────

#[debug_handler]
pub async fn delete(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_order_service): Extension<Arc<OrderService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER"])?;
    let business_id = require_business(&_claims)?;

    _order_service.delete_order(_id, business_id).await?;

    Ok(no_content())
}

// ── GET /orders/analytics?from=2024-01-01&to=2024-12-31 ──────────────────────

#[debug_handler]
pub async fn analytics(
    AuthClaims(_claims): AuthClaims,
    Query(_q): Query<AnalyticsQuery>,
    Extension(_order_service): Extension<Arc<OrderService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "SYSTEM_ADMIN"])?;
    let business_id = require_business(&_claims)?;

    let from = _q.from.as_deref().unwrap_or("");
    let to = _q.to.as_deref().unwrap_or("");

    let result = _order_service.get_analytics(business_id, from, to).await?;

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
