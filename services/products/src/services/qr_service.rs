use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::product::Product;
use crate::repositories::product_repository as repo;
use crate::utils::qr_utils;

fn uploads_dir() -> String {
    std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into())
}

/// Returns the filesystem path to the QR PNG, generating it on-demand if missing.
pub async fn get_qr_path(
    pool: &PgPool,
    id: Uuid,
    farm_id: Uuid,
) -> AppResult<std::path::PathBuf> {
    let product = repo::find_by_id_and_farm(pool, id, farm_id).await?;
    let dir = uploads_dir();

    let relative = match &product.qr_path {
        Some(p) => p.clone(),
        None => {
            let new_path = qr_utils::generate_qr(product.qr_token, &dir)
                .map_err(AppError::Internal)?;
            repo::set_qr_path(pool, product.id, &new_path).await?;
            new_path
        }
    };

    Ok(qr_utils::media_path(&relative, &dir))
}

/// Issues a brand-new QR token PNG, deleting the old file first.
pub async fn regenerate(pool: &PgPool, id: Uuid, farm_id: Uuid) -> AppResult<Product> {
    let product = repo::find_by_id_and_farm(pool, id, farm_id).await?;
    let dir = uploads_dir();

    if let Some(old) = &product.qr_path {
        let _ = std::fs::remove_file(qr_utils::media_path(old, &dir));
    }

    let new_path = qr_utils::generate_qr(product.qr_token, &dir)
        .map_err(AppError::Internal)?;

    repo::set_qr_path_returning(pool, id, farm_id, &new_path).await
}