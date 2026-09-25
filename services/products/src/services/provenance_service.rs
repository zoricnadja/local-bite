use std::sync::Arc;
use uuid::Uuid;


use crate::dtos::provenance_response::ProvenanceResponse;
use crate::models::product::Product;
use crate::repositories::product_repository::ProductRepository;
use common::errors::AppResult;

#[derive(Clone)]
pub struct ProvenanceService {
    pub product_repository: Arc<ProductRepository>,
}
impl ProvenanceService {
    pub fn new(product_repository: Arc<ProductRepository>) -> Self {
        Self { product_repository }
    }

    pub async fn get_provenance(
        &self,
        id: Uuid,
        _business_id: Uuid,
        token: &str,
    ) -> AppResult<ProvenanceResponse> {
        let product = self
            .product_repository
            .find_by_id(id)
            .await?;
        self.build(product, token).await
    }

    pub async fn get_provenance_by_qr(&self, qr_token: Uuid) -> AppResult<ProvenanceResponse> {
        let product = self.product_repository.find_by_qr_token(qr_token).await?;
        let completed:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM production_states WHERE batch_id=$1 AND business_id=$2 AND status='COMPLETED' AND NOT deleted)").bind(product.batch_id).bind(product.business_id).fetch_one(&self.product_repository.pool).await?;
        if !completed || product.expiry_date.is_some_and(|d|d<chrono::Utc::now().date_naive()) { return Err(common::errors::AppError::NotFound("Product is in Storage and is not on sale".into())); }
        self.build(product, "").await
    }

    async fn build(&self, product: Product, _token: &str) -> AppResult<ProvenanceResponse> {
        let token = trace_token(product.business_id, Some(product.id))?;
        let base = std::env::var("READ_MODELS_URL").unwrap_or_else(|_| "http://read-models-service:3006".into());
        let response = reqwest::Client::new().get(format!("{base}/internal/provenance/{}",product.id))
            .bearer_auth(token).timeout(std::time::Duration::from_secs(5)).send().await
            .map_err(|e| common::errors::AppError::Internal(e.into()))?;
        if response.status().as_u16() == 409 {
            return Err(common::errors::AppError::Conflict("Traceability is synchronizing; please retry shortly".into()));
        }
        let response = response.error_for_status().map_err(|e| common::errors::AppError::Internal(e.into()))?;
        #[derive(serde::Deserialize)]
        struct QueryResponse { data: ProvenanceResponse }
        let mut projected = response.json::<QueryResponse>().await
            .map_err(|e| common::errors::AppError::Internal(e.into()))?.data;
        if projected.product.batch_id != product.batch_id {
            return Err(common::errors::AppError::Conflict("Traceability is synchronizing; please retry shortly".into()));
        }
        // Preserve immediate product visibility/QR revocation checks on the owning service.
        // All business/production/material reads come only from the separate query database.
        projected.product = product;
        Ok(projected)
    }
}

// Never sent to the browser: read-only, one business and one batch, expires in a minute.
pub fn trace_token(business_id: Uuid, batch_id: Option<Uuid>) -> AppResult<String> {
    let secret = std::env::var("JWT_SECRET").map_err(|e| common::errors::AppError::Internal(e.into()))?;
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = common::jwt::Claims {
        sub: batch_id.unwrap_or_else(Uuid::nil), email: String::new(),
        role: "TRACEABILITY".into(), business_id: Some(business_id), iat: now, exp: now + 60,
    };
    Ok(jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()))?)
}
