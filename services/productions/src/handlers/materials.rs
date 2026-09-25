use axum::{extract::Path, http::HeaderMap, response::Response, Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

use super::batches::extract_token;
use crate::dtos::raw_material_request::RawMaterialRequest;
use crate::services::raw_materials_service::RawMaterialsService;
use common::{
    errors::AppResult,
    middleware::{require_business, require_role, AuthClaims},
    response::{created, no_content},
};

// ── POST /batches/:id/materials ───────────────────────────────────────────────

pub async fn add_material(
    AuthClaims(_claims): AuthClaims,
    Path(_batch_id): Path<Uuid>,
    headers: HeaderMap,
    Extension(_material_service): Extension<Arc<RawMaterialsService>>,
    Json(req): Json<RawMaterialRequest>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let _business_id = require_business(&_claims)?;
    let token = extract_token(&headers);
    Ok(created(
        _material_service
            .add(_batch_id, _business_id, req, &token)
            .await?,
    ))
}

// ── DELETE /batches/:id/materials/:material_id ────────────────────────────────

pub async fn remove_material(
    AuthClaims(_claims): AuthClaims,
    Path((_batch_id, _raw_material_id)): Path<(Uuid, Uuid)>,
    Extension(_material_service): Extension<Arc<RawMaterialsService>>,
) -> AppResult<Response> {
    require_role(&_claims, &["BUSINESS_OWNER", "WORKER"])?;
    let _business_id = require_business(&_claims)?;
    _material_service
        .remove(_batch_id, _raw_material_id, _business_id)
        .await?;
    Ok(no_content())
}
