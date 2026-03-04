use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;
use crate::dtos::create_product_request::CreateProductRequest;
use crate::dtos::update_product_request::UpdateProductRequest;
use crate::models::insert_product_params::InsertParams;
use crate::models::product::Product;
use crate::models::query::ListQuery;
use crate::models::update_product_params::UpdateParams;
use crate::repositories::product_repository::ProductRepository;
use crate::utils::qr_utils;

fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

#[derive(Clone)]
pub struct ProductService {
    pub product_repository: Arc<ProductRepository>,
    uploads_dir: String,
}

impl ProductService {
    pub fn new(product_repository: Arc<ProductRepository>, uploads_dir: String) -> Self {
        Self { product_repository, uploads_dir }
    }

    pub async fn find_all_by_farm_id(
        &self,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<PaginatedResponse<Product>> {
        let (items, total) = self.product_repository.find_all_by_farm_id(farm_id, q).await?;
        Ok(PaginatedResponse {
            data: items,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }
    pub async fn find_all(
        &self,
        q: &ListQuery,
    ) -> AppResult<PaginatedResponse<Product>> {
        let (items, total) = self.product_repository.find_all(q).await?;
        Ok(PaginatedResponse {
            data: items,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    pub async fn create(
        &self,
        farm_id: Uuid,
        req: CreateProductRequest,
    ) -> AppResult<Product> {
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("Name cannot be empty".into()));
        }
        if req.price < 0.0 {
            return Err(AppError::BadRequest("Price cannot be negative".into()));
        }
        if req.quantity < 0.0 {
            return Err(AppError::BadRequest("Quantity cannot be negative".into()));
        }

        let id = Uuid::new_v4();
        let qr_token = Uuid::new_v4();
        let qr_path = qr_utils::generate_qr(qr_token, &self.uploads_dir).map(Some).unwrap_or(None);

        self.product_repository.insert(InsertParams {
            id,
            farm_id,
            name: req.name.trim().to_string(),
            product_type: req.product_type.trim().to_string(),
            description: req.description,
            quantity: dec(req.quantity),
            unit: req.unit.trim().to_string(),
            price: dec(req.price),
            batch_id: req.batch_id,
            qr_token,
            qr_path,
        })
            .await
    }

    pub async fn get_one(&self, id: Uuid) -> AppResult<Product> {
        self.product_repository.find_by_id(id).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        farm_id: Uuid,
        req: UpdateProductRequest,
    ) -> AppResult<Product> {
        let existing = self.product_repository.find_by_id_and_farm(id, farm_id).await?;

        self.product_repository.update(id, farm_id, UpdateParams {
            name: req.name.as_deref().unwrap_or(&existing.name).to_string(),
            product_type: req.product_type.as_deref().unwrap_or(&existing.product_type).to_string(),
            description: req.description.as_deref().or(existing.description.as_deref()).map(str::to_string),
            quantity: req.quantity.map(dec).unwrap_or(existing.quantity),
            unit: req.unit.as_deref().unwrap_or(&existing.unit).to_string(),
            price: req.price.map(dec).unwrap_or(existing.price),
            batch_id: req.batch_id.or(existing.batch_id),
            is_active: req.is_active.unwrap_or(existing.is_active),
        })
            .await
    }

    pub async fn delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let rows = self.product_repository.soft_delete(id, farm_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!("Product {} not found", id)));
        }
        Ok(())
    }

    pub async fn decrement(
        &self,
        id: Uuid,
        amount: f64,
    ) -> AppResult<Product> {
        if amount <= 0.0 {
            return Err(AppError::BadRequest("Decrement amount must be > 0".into()));
        }

        let product = self.product_repository.find_by_id(id).await?;

        let new_qty = &product.quantity - &dec(amount);
        if new_qty < BigDecimal::from(0) {
            return Err(AppError::BadRequest(format!(
                "Insufficient stock for '{}': available {}, requested {}",
                product.name, product.quantity, amount
            )));
        }

        self.product_repository.update_quantity(id, new_qty).await
    }
}