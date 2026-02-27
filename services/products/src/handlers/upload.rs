use axum::{
    extract::{Multipart, Path, State},
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use common::{
    errors::{AppError, AppResult},
    middleware::{require_farm, require_role, AuthClaims},
    response::ok,
};
use crate::service::image_service;
use super::products::fetch_owned;

fn uploads_dir() -> String {
    std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into())
}

// ── POST /products/:id/image ──────────────────────────────────────────────────

pub async fn upload_image(
    AuthClaims(claims): AuthClaims,
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> AppResult<Response> {
    require_role(&claims, &["FARM_OWNER", "WORKER"])?;
    let farm_id = require_farm(&claims)?;

    // Verify ownership before accepting the upload
    let existing = fetch_owned(&pool, id, farm_id).await?;

    // Extract the file field from multipart
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut mime_type = String::from("application/octet-stream");

    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError::BadRequest(e.to_string()))? 
    {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name == "image" {
            mime_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();

            file_bytes = Some(
                field.bytes().await
                    .map_err(|e| AppError::BadRequest(format!("Failed to read upload: {}", e)))?
                    .to_vec(),
            );
            break;
        }
    }

    let bytes = file_bytes.ok_or_else(|| AppError::BadRequest("No 'image' field found in form".into()))?;

    // Validate MIME type
    if !image_service::is_allowed_mime(&mime_type) {
        return Err(AppError::BadRequest(
            "Only JPEG, PNG and WebP images are allowed".into(),
        ));
    }

    // Delete old image if one exists
    if let Some(old_path) = &existing.image_path {
        image_service::delete_image(old_path, &uploads_dir());
    }

    // Save new image
    let relative_path = image_service::save_image(&bytes, &uploads_dir())
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Persist path in DB
    let updated = sqlx::query_as!(
        Product,
        r#"
        UPDATE products SET image_path = $1
        WHERE id = $2 AND farm_id = $3
        RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                  batch_id, image_path, qr_token, qr_path, is_active, is_deleted,
                  created_at, updated_at
        "#,
        relative_path, id, farm_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(ok(updated))
}
