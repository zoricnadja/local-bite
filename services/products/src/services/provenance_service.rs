use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppResult;
use crate::dtos::clients;
use crate::dtos::provenance_response::ProvenanceResponse;
use crate::models::product::Product;
use crate::repositories::product_repository as repo;

pub async fn get_provenance(
    pool: &PgPool,
    id: Uuid,
    farm_id: Uuid,
    token: &str,
) -> AppResult<ProvenanceResponse> {
    let product = repo::find_by_id_and_farm(pool, id, farm_id).await?;
    build(product, token).await
}

pub async fn get_provenance_by_qr(pool: &PgPool, qr_token: Uuid) -> AppResult<ProvenanceResponse> {
    let product = repo::find_by_qr_token(pool, qr_token).await?;
    build(product, "").await
}

async fn build(product: Product, token: &str) -> AppResult<ProvenanceResponse> {
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