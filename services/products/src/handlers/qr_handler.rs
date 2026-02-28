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
use crate::services::qr_service;

// ── GET /products/:id/qr ──────────────────────────────────────────────────────

pub async fn get_qr(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER", "CUSTOMER", "SYSTEM_ADMIN"])?;
    let farm_id = require_farm(&claims)?;

    let path = qr_service::get_qr_path(&pool, id, farm_id).await?;
    serve_png_file(&path).await
}

// ── POST /products/:id/qr/regenerate ─────────────────────────────────────────

pub async fn regenerate_qr(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER"])?;
    let farm_id = require_farm(&claims)?;

    let updated = qr_service::regenerate(&pool, id, farm_id).await?;
    Ok(ok(updated))
}

// ── Private ───────────────────────────────────────────────────────────────────

async fn serve_png_file(path: &std::path::Path) -> AppResult<Response> {
    let file = File::open(path)
        .await
        .map_err(|_| AppError::NotFound("QR file not found on disk".into()))?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        Body::from_stream(ReaderStream::new(file)),
    )
        .into_response())
}