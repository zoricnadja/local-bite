use axum::{debug_handler, extract::Path, response::Response, Extension};
use std::sync::Arc;
use uuid::Uuid;

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