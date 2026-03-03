use std::sync::Arc;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::models::product::Product;
use crate::repositories::product_repository::ProductRepository;
use crate::utils::qr_utils;

#[derive(Clone)]
pub struct QrService {
    pub product_repository: Arc<ProductRepository>,
    uploads_dir: String,
}

impl QrService {
    pub fn new(product_repository: Arc<ProductRepository>, uploads_dir: String) -> Self {
        Self { product_repository, uploads_dir }
    }
    
    /// Returns the filesystem path to the QR PNG, generating it on-demand if missing.
    pub async fn get_qr_path(
        &self,
        id: Uuid,
    ) -> AppResult<std::path::PathBuf> {
        let product = self.product_repository.find_by_id(id).await?;
        let relative = match &product.qr_path {
            Some(p) => p.clone(),
            None => {
                let new_path = qr_utils::generate_qr(product.qr_token, &self.uploads_dir)
                    .map_err(AppError::Internal)?;
                self.product_repository.set_qr_path(product.id, &new_path).await?;
                new_path
            }
        };
    
        Ok(qr_utils::media_path(&relative, &self.uploads_dir))
    }
    
    /// Issues a brand-new QR token PNG, deleting the old file first.
    pub async fn regenerate(&self, id: Uuid) -> AppResult<Product> {
        let product = self.product_repository.find_by_id(id).await?;

        if let Some(old) = &product.qr_path {
            let _ = std::fs::remove_file(qr_utils::media_path(old, &self.uploads_dir));
        }
    
        let new_path = qr_utils::generate_qr(product.qr_token, &self.uploads_dir)
            .map_err(AppError::Internal)?;
    
        self.product_repository.set_qr_path_returning(id, product.farm_id, &new_path).await
    }
}