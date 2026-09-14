use axum::{
    body::Body,
    debug_handler,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::certificate_service::CertificateService;
use crate::services::provenance_service::ProvenanceService;
use common::{errors::AppResult, response::ok};
// ── GET /products/public/:qr_token  — no auth required ───────────────────────

#[debug_handler]
pub async fn scan(
    Path(_qr_token): Path<Uuid>,
    Extension(_provenance_service): Extension<Arc<ProvenanceService>>,
) -> AppResult<Response> {
    let result = _provenance_service.get_provenance_by_qr(_qr_token).await?;
    Ok(ok(result))
}

/// Public, immutable-at-request-time certificate. The QR token is deliberately
/// used instead of a product id so that a customer never needs an account.
pub async fn certificate(
    Path(qr_token): Path<Uuid>,
    Extension(certificate_service): Extension<Arc<CertificateService>>,
) -> AppResult<Response> {
    let bytes = certificate_service.generate(qr_token).await?;
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "inline; filename=local-bite-certificate.pdf",
            ),
        ],
        Body::from(bytes),
    )
        .into_response())
}
