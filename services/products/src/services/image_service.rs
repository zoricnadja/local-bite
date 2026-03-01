use std::sync::Arc;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::product::Product;
use crate::repositories::product_repository::ProductRepository;
use crate::utils::image_utils;

#[derive(Clone)]
pub struct ImageService {
    pub product_repository: Arc<ProductRepository>,
    uploads_dir: String,
}

impl ImageService {
    pub fn new(product_repository: Arc<ProductRepository>, uploads_dir: String) -> Self {
        Self { product_repository, uploads_dir }
    }

    pub async fn upload(
        &self,
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

        let existing = self.product_repository.find_by_id_and_farm(id, farm_id).await?;

        if let Some(old_path) = &existing.image_path {
            image_utils::delete_image(old_path, &self.uploads_dir);
        }

        let relative_path = image_utils::save_image(&bytes, &self.uploads_dir)
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        self.product_repository.set_image_path(id, farm_id, &relative_path).await
    }
}