use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::Response,
    Json,
};
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use common::{
    errors::{AppError, AppResult},
    middleware::{require_farm, require_role, AuthClaims},
    response::{created, no_content, ok},
};
use common::paginated_response::PaginatedResponse;
use crate::service::qr_service;
use crate::dtos::clients;
use crate::dtos::create_product_request::CreateProductRequest;
use crate::dtos::provenance_response::ProvenanceResponse;
use crate::dtos::update_product_request::UpdateProductRequest;
use crate::models::product::Product;
use crate::models::query::ListQuery;

fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

fn uploads_dir() -> String {
    std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into())
}

// ── GET /products ─────────────────────────────────────────────────────────────

pub async fn list(
    AuthClaims(claims): AuthClaims,
    Query(q): Query<ListQuery>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;

    let offset      = q.offset();
    let limit       = q.limit();
    let type_filter = q.product_type.as_deref().unwrap_or("");
    let search      = q.search.as_deref().unwrap_or("");
    let active_only = q.active_only.unwrap_or(false);

    let items = sqlx::query_as!(
        Product,
        r#"
        SELECT id, farm_id, name, product_type, description, quantity, unit, price,
               batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
               created_at, updated_at
        FROM   products
        WHERE  farm_id    = $1
          AND  is_deleted = FALSE
          AND  ($2 = '' OR product_type ILIKE $2)
          AND  ($3 = '' OR name ILIKE '%' || $3 || '%')
          AND  (NOT $4    OR is_active = TRUE)
        ORDER  BY created_at DESC
        LIMIT  $5 OFFSET $6
        "#,
        farm_id, type_filter, search, active_only, limit, offset
    )
        .fetch_all(&pool)
        .await?;

    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM products
        WHERE  farm_id = $1 AND is_deleted = FALSE
          AND  ($2 = '' OR product_type ILIKE $2)
          AND  ($3 = '' OR name ILIKE '%' || $3 || '%')
          AND  (NOT $4    OR is_active = TRUE)
        "#,
        farm_id, type_filter, search, active_only
    )
        .fetch_one(&pool)
        .await?
        .unwrap_or(0);

    Ok(ok(PaginatedResponse {
        data: items,
        total,
        page: q.page.unwrap_or(1),
        limit,
    }))
}

// ── POST /products ────────────────────────────────────────────────────────────

pub async fn create(
    AuthClaims(claims): AuthClaims,
    State(pool): State<PgPool>,
    Json(req): Json<CreateProductRequest>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;

    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".into()));
    }
    if req.price < 0.0 {
        return Err(AppError::BadRequest("Price cannot be negative".into()));
    }
    if req.quantity < 0.0 {
        return Err(AppError::BadRequest("Quantity cannot be negative".into()));
    }

    let id        = Uuid::new_v4();
    let qr_token  = Uuid::new_v4();

    // Pre-generate QR code
    let qr_path = qr_service::generate_qr(qr_token, &uploads_dir())
        .map(Some)
        .unwrap_or(None);

    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products
            (id, farm_id, name, product_type, description, quantity, unit,
             price, batch_id, qr_token, qr_path)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
        RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                  batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                  created_at, updated_at
        "#,
        id,
        farm_id,
        req.name.trim(),
        req.product_type.trim(),
        req.description.as_deref(),
        dec(req.quantity),
        req.unit.trim(),
        dec(req.price),
        req.batch_id,
        qr_token,
        qr_path,
    )
        .fetch_one(&pool)
        .await?;

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

    let product = fetch_owned(&pool, id, farm_id).await?;
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

    let product = fetch_owned(&pool, id, farm_id).await?;

    // Forward the original JWT for downstream service calls
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or_default()
        .to_string();

    // Fetch farm name and batch info concurrently
    let (farm_name, batch) = tokio::join!(
        clients::fetch_farm_name(product.farm_id, &token),
        async {
            if let Some(bid) = product.batch_id {
                clients::fetch_batch(bid, &token).await.ok()
            } else {
                None
            }
        }
    );

    Ok(ok(ProvenanceResponse { product, farm_name, batch }))
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

    let existing = fetch_owned(&pool, id, farm_id).await?;

    let new_name     = req.name.as_deref().unwrap_or(&existing.name).to_string();
    let new_type     = req.product_type.as_deref().unwrap_or(&existing.product_type).to_string();
    let new_desc     = req.description.as_deref().or(existing.description.as_deref());
    let new_quantity = req.quantity.map(dec).unwrap_or(existing.quantity);
    let new_unit     = req.unit.as_deref().unwrap_or(&existing.unit).to_string();
    let new_price    = req.price.map(dec).unwrap_or(existing.price);
    let new_batch    = req.batch_id.or(existing.batch_id);
    let new_active   = req.is_active.unwrap_or(existing.is_active);

    let updated = sqlx::query_as!(
        Product,
        r#"
        UPDATE products SET
            name         = $1,
            product_type = $2,
            description  = $3,
            quantity     = $4,
            unit         = $5,
            price        = $6,
            batch_id     = $7,
            is_active    = $8
        WHERE id = $9 AND farm_id = $10 AND is_deleted = FALSE
        RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                  batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                  created_at, updated_at
        "#,
        new_name, new_type, new_desc, new_quantity,
        new_unit, new_price, new_batch, new_active,
        id, farm_id
    )
        .fetch_one(&pool)
        .await?;

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

    let rows = sqlx::query!(
        "UPDATE products SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2 AND is_deleted = FALSE",
        id, farm_id
    )
        .execute(&pool)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Product {} not found", id)));
    }

    Ok(no_content())
}

// ── Private ───────────────────────────────────────────────────────────────────

pub async fn fetch_owned(pool: &PgPool, id: Uuid, farm_id: Uuid) -> AppResult<Product> {
    sqlx::query_as!(
        Product,
        r#"
        SELECT id, farm_id, name, product_type, description, quantity, unit, price,
               batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
               created_at, updated_at
        FROM   products
        WHERE  id = $1 AND farm_id = $2 AND is_deleted = FALSE
        "#,
        id, farm_id
    )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))
}
