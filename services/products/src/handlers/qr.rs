use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use common::{
    errors::{AppError, AppResult},
    middleware::{require_farm, require_role, AuthClaims},
    response::ok,
};
use crate::models::product::Product;
use crate::service::qr_service;
use super::products::fetch_owned;

fn uploads_dir() -> String {
    std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into())
}

// ── GET /products/:id/qr  — returns the QR PNG as a file download ─────────────

pub async fn get_qr(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "CUSTOMER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;

    let product = fetch_owned(&pool, id, farm_id).await?;

    let qr_path = ensure_qr(&pool, &product, &uploads_dir()).await?;
    serve_png_file(&qr_path).await
}

// ── POST /products/:id/qr/regenerate ─────────────────────────────────────────

pub async fn regenerate_qr(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&claims)?;

    let product = fetch_owned(&pool, id, farm_id).await?;

    // Delete old file if exists
    if let Some(old) = &product.qr_path {
        let _ = std::fs::remove_file(qr_service::media_path(old, &uploads_dir()));
    }

    let new_path = qr_service::generate_qr(product.qr_token, &uploads_dir())
        .map_err(|e| AppError::Internal(e))?;

    let updated = sqlx::query_as!(
        Product,
        r#"
        UPDATE products SET qr_path = $1 WHERE id = $2 AND farm_id = $3
        RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                  batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                  created_at, updated_at
        "#,
        new_path, id, farm_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(ok(updated))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns the full path to the QR PNG, generating it on-demand if missing.
async fn ensure_qr(pool: &PgPool, product: &Product, uploads_dir: &str) -> AppResult<std::path::PathBuf> {
    let relative = match &product.qr_path {
        Some(p) => p.clone(),
        None => {
            // Generate on-demand and persist
            let new_path = qr_service::generate_qr(product.qr_token, uploads_dir)
                .map_err(|e| AppError::Internal(e))?;

            sqlx::query!(
                "UPDATE products SET qr_path = $1 WHERE id = $2",
                new_path, product.id
            )
            .execute(pool)
            .await?;

            new_path
        }
    };

    Ok(qr_service::media_path(&relative, uploads_dir))
}

async fn serve_png_file(path: &std::path::Path) -> AppResult<Response> {
    let file = File::open(path).await
        .map_err(|_| AppError::NotFound("QR file not found on disk".into()))?;

    let stream = ReaderStream::new(file);
    let body   = Body::from_stream(stream);

    let response = (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        body,
    )
    .into_response();

    Ok(response)
}
