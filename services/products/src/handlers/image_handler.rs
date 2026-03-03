use axum::{
    debug_handler,
    extract::{Multipart, Path},
    response::Response,
    Extension,
};
use std::sync::Arc;
use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::services::image_service::ImageService;
use common::{
    errors::{AppError, AppResult},
    middleware::{require_farm, require_role, AuthClaims},
    response::ok,
};

// ── POST /products/:id/image ──────────────────────────────────────────────────

#[debug_handler]
pub async fn upload_image(
    AuthClaims(_claims): AuthClaims,
    Path(_id): Path<Uuid>,
    Extension(_image_service): Extension<Arc<ImageService>>,
    mut multipart: Multipart,
) -> AppResult<Response> {
    require_role(&_claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&_claims)?;

    let mut file_bytes: Option<Vec<u8>> = None;
    let mut mime_type = String::from("application/octet-stream");

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        if field.name().unwrap_or("") == "image" {
            mime_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            file_bytes = Some(
                field
                    .bytes()
                    .await
                    .map_err(|e| AppError::BadRequest(format!("Failed to read upload: {}", e)))?
                    .to_vec(),
            );
            break;
        }
    }

    let bytes = file_bytes
        .ok_or_else(|| AppError::BadRequest("No 'image' field found in form".into()))?;

    let updated = _image_service.upload(_id, farm_id, bytes, &mime_type).await?;
    Ok(ok(updated))
}

// ── GET /products/:id/image ──────────────────────────────────────────────────

#[debug_handler]
pub async fn get_image(
    Path(_id): Path<Uuid>,
    Extension(_image_service): Extension<Arc<ImageService>>,
) -> AppResult<Response> {
    let path = _image_service.get_image_path(_id).await?;
    serve_file(path.as_ref()).await
}


async fn serve_file(full_path: &std::path::Path) -> AppResult<Response> {

    let file = File::open(&full_path)
        .await
        .map_err(|_| AppError::NotFound("Image not found on disk".into()))?;
    let content_type = match full_path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type)],
        Body::from_stream(ReaderStream::new(file)),
    )
        .into_response())
}