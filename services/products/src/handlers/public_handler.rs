use axum::{
    extract::{Path, State},
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use common::{errors::AppResult, response::ok};
use crate::services::provenance_service;

// ── GET /products/public/:qr_token  — no auth required ───────────────────────

pub async fn scan(
    Path(qr_token): Path<Uuid>,
    State(pool): State<PgPool>,
) -> AppResult<Response> {
    let result = provenance_service::get_provenance_by_qr(&pool, qr_token).await?;
    Ok(ok(result))
}