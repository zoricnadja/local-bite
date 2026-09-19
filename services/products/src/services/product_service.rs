use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::create_product_request::CreateProductRequest;
use crate::dtos::update_product_request::UpdateProductRequest;

use crate::models::product::Product;
use crate::models::query::ListQuery;
use crate::models::update_product_params::UpdateParams;
use crate::repositories::product_repository::ProductRepository;
use crate::services::product_policy::ProductPolicyFactory;

use chrono::Utc;
use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;


fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

#[derive(Clone)]
pub struct ProductService {
    pub product_repository: Arc<ProductRepository>,

}


impl ProductService {
    pub fn new(product_repository: Arc<ProductRepository>) -> Self {
        Self {
            product_repository,

        }
    }

    pub async fn find_all_by_farm_id(
        &self,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<PaginatedResponse<Product>> {
        let (items, total) = self
            .product_repository
            .find_all_by_farm_id(farm_id, q)
            .await?;
        Ok(PaginatedResponse {
            data: items,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }
    pub async fn find_all(&self, q: &ListQuery) -> AppResult<PaginatedResponse<Product>> {
        let (items, total) = self.product_repository.find_all(q).await?;
        Ok(PaginatedResponse {
            data: items,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    pub async fn create(&self, farm_id: Uuid, req: CreateProductRequest) -> AppResult<Product> {
        let _ = (farm_id,req);
        Err(AppError::BadRequest("Complete a production batch to create its output in Storage".into()))
    }

    pub async fn get_one(&self, id: Uuid) -> AppResult<Product> {
        let mut product=self.product_repository.find_by_id(id).await?;
        let completed:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM production_states WHERE batch_id=$1 AND farm_id=$2 AND status='COMPLETED' AND NOT deleted)").bind(product.batch_id).bind(product.farm_id).fetch_one(&self.product_repository.pool).await?;
        product.is_active &= completed && product.expiry_date.is_none_or(|d|d>=Utc::now().date_naive());
        Ok(product)
    }

    pub async fn update(
        &self,
        id: Uuid,
        farm_id: Uuid,
        mut req: UpdateProductRequest,
    ) -> AppResult<Product> {
        let existing = self
            .product_repository
            .find_by_id_and_farm(id, farm_id)
            .await?;

        if existing.status == "PRODUCTION" { return Err(AppError::BadRequest("Edit planned products through their production batch".into())); }
        if let Some(status) = &req.status {
            if !["STORAGE", "ON_SALE"].contains(&status.as_str()) { return Err(AppError::BadRequest("Invalid product state".into())); }
            req.is_active = Some(status == "ON_SALE");
        }
        ProductPolicyFactory::for_type(req.product_type.as_deref().unwrap_or(&existing.product_type)).validate(&CreateProductRequest {
            name:req.name.clone().unwrap_or_else(||existing.name.clone()),product_type:req.product_type.clone().unwrap_or_else(||existing.product_type.clone()),description:req.description.clone(),
            quantity:req.quantity.unwrap_or_else(||existing.quantity.to_string().parse().unwrap_or(0.0)),
            unit:req.unit.clone().unwrap_or_else(||existing.unit.clone()),price:req.price.unwrap_or_else(||existing.price.to_string().parse().unwrap_or(0.0)),expiry_date:req.expiry_date.or(existing.expiry_date),batch_id:req.batch_id.or(existing.batch_id)
        })?;
        if req.quantity.is_some_and(|v|!v.is_finite() || v<0.0){return Err(AppError::BadRequest("Invalid quantity".into()));}
        if req.is_active.unwrap_or(existing.is_active) {
            self.validate_batch(farm_id, req.batch_id.or(existing.batch_id), req.expiry_date.or(existing.expiry_date)).await?;
            if !req.price.as_ref().map_or_else(|| existing.price > BigDecimal::from(0), |p| p.is_finite() && *p>0.0) { return Err(AppError::BadRequest("Set a positive price before placing a product on sale".into())); }
        }
        let measured:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM production_outputs WHERE product_id=$1)").bind(id).fetch_one(&self.product_repository.pool).await?;
        if measured && (req.batch_id.is_some_and(|b| Some(b)!=existing.batch_id) || req.unit.as_ref().is_some_and(|u| *u!=existing.unit) || req.quantity.is_some_and(|q| dec(q)!=existing.quantity)) {
            return Err(AppError::BadRequest("Measured production quantity, unit and batch cannot be changed on a product".into()));
        }
        if req.price.is_some_and(|p| !p.is_finite() || p<0.0) { return Err(AppError::BadRequest("Price must be a non-negative number".into())); }

        self.product_repository
            .update(
                id,
                farm_id,
                UpdateParams {
                    name: req.name.as_deref().unwrap_or(&existing.name).to_string(),
                    product_type: req
                        .product_type
                        .as_deref()
                        .unwrap_or(&existing.product_type)
                        .to_string(),
                    description: req
                        .description
                        .as_deref()
                        .or(existing.description.as_deref())
                        .map(str::to_string),
                    quantity: req.quantity.map(dec),
                    unit: req.unit.as_deref().unwrap_or(&existing.unit).to_string(),
                    price: req.price.map(dec).unwrap_or(existing.price),
                    expiry_date: req.expiry_date.or(existing.expiry_date),
                    batch_id: req.batch_id.or(existing.batch_id),
                    status: if req.is_active.unwrap_or(existing.is_active) { "ON_SALE" } else { "STORAGE" }.into(),
                    is_active: req.is_active.unwrap_or(existing.is_active),
                },
            )
            .await
    }

    pub async fn delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let product = self.product_repository.find_by_id_and_farm(id, farm_id).await?;
        if product.status == "PRODUCTION" {
            return Err(AppError::BadRequest("Remove planned products through their production batch".into()));
        }
        let rows = self.product_repository.soft_delete(id, farm_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!("Product {} not found", id)));
        }
        Ok(())
    }

    async fn validate_batch(&self, farm_id: Uuid, batch_id: Option<Uuid>, expiry: Option<chrono::NaiveDate>) -> AppResult<()> {
        if batch_id.is_none() { return Err(AppError::BadRequest("Link a completed production batch before placing the product on sale".into())); }
        if let Some(id) = batch_id {
            let token = super::provenance_service::trace_token(farm_id, Some(id))?;
            let batch = crate::dtos::clients::fetch_batch(id, &token).await
                .map_err(|_| AppError::BadRequest("Select an available production batch from your farm".into()))?;
            if batch.status != "COMPLETED" {
                return Err(AppError::BadRequest("Production must be completed before the product can be active".into()));
            }
            if let (Some(expiry), Some(end)) = (expiry, batch.end_date) {
                if let Ok(end) = chrono::NaiveDate::parse_from_str(&end, "%Y-%m-%d") {
                    if expiry < end {
                        return Err(AppError::BadRequest("Product expiry cannot be before production ends".into()));
                    }
                }
            }
        }
        Ok(())
    }


}

pub fn authorize_read(product:&Product,claims:&common::jwt::Claims)->AppResult<()> {
    let allowed=match claims.role.as_str(){"SYSTEM_ADMIN"=>true,"CUSTOMER"=>product.is_active && product.expiry_date.is_none_or(|d|d>=Utc::now().date_naive()),"FARM_OWNER"|"WORKER"=>claims.farm_id==Some(product.farm_id),_=>false};
    if allowed {Ok(())}else{Err(AppError::NotFound("Product not found".into()))}
}
