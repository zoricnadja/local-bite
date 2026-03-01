use std::sync::Arc;
use uuid::Uuid;

use common::errors::AppResult;
use crate::dtos::clients;
use crate::dtos::provenance_response::ProvenanceResponse;
use crate::models::product::Product;
use crate::repositories::product_repository::ProductRepository;

#[derive(Clone)]
pub struct ProvenanceService {
    pub product_repository: Arc<ProductRepository>
}
impl ProvenanceService {
    pub fn new(product_repository: Arc<ProductRepository>) -> Self {
        Self { product_repository }
    }

    pub async fn get_provenance(
        &self,
        id: Uuid,
        farm_id: Uuid,
        token: &str,
    ) -> AppResult<ProvenanceResponse> {
        let product = self.product_repository.find_by_id_and_farm(id, farm_id).await?;
        self.build(product, token).await
    }

    pub async fn get_provenance_by_qr(&self, qr_token: Uuid) -> AppResult<ProvenanceResponse> {
        let product = self.product_repository.find_by_qr_token(qr_token).await?;
        self.build(product, "").await
    }

    async fn build(&self, product: Product, token: &str) -> AppResult<ProvenanceResponse> {
        let (farm_name, batch) = tokio::join!(
            clients::fetch_farm_name(product.farm_id, token),
            async {
                if let Some(bid) = product.batch_id {
                    clients::fetch_batch(bid, token).await.ok()
                } else {
                    None
                }
            }
        );
        Ok(ProvenanceResponse { product, farm_name, batch })
    }
}