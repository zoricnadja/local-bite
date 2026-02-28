use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::product::Product;
use crate::repositories::product_repository as repo;
use crate::utils::image_utils;

fn uploads_dir() -> String {
    std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into())
}

pub async fn upload(
    pool: &PgPool,
    id: Uuid,
    farm_id: Uuid,
    bytes: Vec<u8>,
    mime_type: &str,
) -> AppResult<Product> {
    if !image_utils::is_allowed_mime(mime_type) {
        return Err(AppError::BadRequest(
            "Only JPEG, PNG and WebP images are allowed".into(),
        ));
    }

    let existing = repo::find_by_id_and_farm(pool, id, farm_id).await?;
    let dir = uploads_dir();

    if let Some(old_path) = &existing.image_path {
        image_utils::delete_image(old_path, &dir);
    }

    let relative_path = image_utils::save_image(&bytes, &dir)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    repo::set_image_path(pool, id, farm_id, &relative_path).await
}