use bigdecimal::BigDecimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;
use crate::dtos::create_product_request::CreateProductRequest;
use crate::dtos::update_product_request::UpdateProductRequest;
use crate::models::product::Product;
use crate::models::query::ListQuery;
use crate::repositories::product_repository as repo;
use crate::utils::qr_utils;

fn dec(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

fn uploads_dir() -> String {
    std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "./uploads".into())
}

pub async fn list(
    pool: &PgPool,
    farm_id: Uuid,
    q: &ListQuery,
) -> AppResult<PaginatedResponse<Product>> {
    let (items, total) = repo::list(pool, farm_id, q).await?;
    Ok(PaginatedResponse {
        data: items,
        total,
        page: q.page.unwrap_or(1),
        limit: q.limit(),
    })
}

pub async fn create(
    pool: &PgPool,
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

    let id       = Uuid::new_v4();
    let qr_token = Uuid::new_v4();
    let qr_path  = qr_utils::generate_qr(qr_token, &uploads_dir()).map(Some).unwrap_or(None);

    repo::insert(pool, repo::InsertParams {
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

pub async fn get_one(pool: &PgPool, id: Uuid, farm_id: Uuid) -> AppResult<Product> {
    repo::find_by_id_and_farm(pool, id, farm_id).await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    farm_id: Uuid,
    req: UpdateProductRequest,
) -> AppResult<Product> {
    let existing = repo::find_by_id_and_farm(pool, id, farm_id).await?;

    repo::update(pool, id, farm_id, repo::UpdateParams {
        name:         req.name.as_deref().unwrap_or(&existing.name).to_string(),
        product_type: req.product_type.as_deref().unwrap_or(&existing.product_type).to_string(),
        description:  req.description.as_deref().or(existing.description.as_deref()).map(str::to_string),
        quantity:     req.quantity.map(dec).unwrap_or(existing.quantity),
        unit:         req.unit.as_deref().unwrap_or(&existing.unit).to_string(),
        price:        req.price.map(dec).unwrap_or(existing.price),
        batch_id:     req.batch_id.or(existing.batch_id),
        is_active:    req.is_active.unwrap_or(existing.is_active),
    })
        .await
}

pub async fn delete(pool: &PgPool, id: Uuid, farm_id: Uuid) -> AppResult<()> {
    let rows = repo::soft_delete(pool, id, farm_id).await?;
    if rows == 0 {
        return Err(AppError::NotFound(format!("Product {} not found", id)));
    }
    Ok(())
}