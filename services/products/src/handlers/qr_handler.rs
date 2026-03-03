use axum::{body::Body, extract::Path, http::{header, StatusCode}, response::{IntoResponse, Response}, Extension};
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::services::qr_service::QrService;
use common::{
    errors::{AppError, AppResult},
    middleware::{require_role, AuthClaims},
    response::ok,
};
// ── GET /products/:id/qr ──────────────────────────────────────────────────────

pub async fn get_qr(
    Path(id): Path<Uuid>,
    Extension(_qr_service): Extension<Arc<QrService>>,
) -> AppResult<Response> {

    let path = _qr_service.get_qr_path(id).await?;
    serve_png_file(&path).await
}

// ── POST /products/:id/qr/regenerate ─────────────────────────────────────────

pub async fn regenerate_qr(
    AuthClaims(_claims): AuthClaims,
    Path(id): Path<Uuid>,
    Extension(_qr_service): Extension<Arc<QrService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER"])?;

    let updated = _qr_service.regenerate(id).await?;
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