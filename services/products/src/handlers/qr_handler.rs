use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
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
    AuthClaims(claims): AuthClaims,
    Extension(products): Extension<Arc<crate::services::product_service::ProductService>>,
    Path(id): Path<Uuid>,
    Extension(_qr_service): Extension<Arc<QrService>>,
) -> AppResult<Response> {
    let product = products.get_one(id).await?;
    crate::services::product_service::authorize_read(&product,&claims)?;
    let path = _qr_service.get_qr_path(id).await?;
    serve_png_file(&path).await
}

// ── POST /products/:id/qr/regenerate ─────────────────────────────────────────

pub async fn regenerate_qr(
    AuthClaims(_claims): AuthClaims,
    Path(id): Path<Uuid>,
    Extension(_qr_service): Extension<Arc<QrService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER"])?;

    let business=common::middleware::require_business(&_claims)?;
    _qr_service.product_repository.find_by_id_and_business(id,business).await?;
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
