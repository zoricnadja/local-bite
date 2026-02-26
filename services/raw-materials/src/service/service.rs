use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use common::errors::{AppError, AppResult};

use crate::{
    models::models::{
        AdjustQuantityRequest, CreateRawMaterialRequest, ListQuery, PaginatedResponse,
        RawMaterial, UpdateRawMaterialRequest,
    },
    repository::repository::RawMaterialRepository,
};

fn to_decimal(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

pub struct RawMaterialService;

impl RawMaterialService {
    pub async fn list(
        pool: &sqlx::PgPool,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<PaginatedResponse<RawMaterial>> {
        let (data, total) = RawMaterialRepository::find_all(pool, farm_id, q).await?;
        Ok(PaginatedResponse {
            data,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        farm_id: Uuid,
        req: CreateRawMaterialRequest,
    ) -> AppResult<RawMaterial> {
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("Name cannot be empty".into()));
        }
        if req.quantity < 0.0 {
            return Err(AppError::BadRequest("Quantity cannot be negative".into()));
        }
        if req.unit.trim().is_empty() {
            return Err(AppError::BadRequest("Unit cannot be empty".into()));
        }

        RawMaterialRepository::insert(
            pool,
            Uuid::new_v4(),
            farm_id,
            req.name.trim(),
            req.material_type.trim(),
            to_decimal(req.quantity),
            req.unit.trim(),
            req.supplier.as_deref(),
            req.origin.as_deref(),
            req.harvest_date,
            req.expiry_date,
            req.notes.as_deref(),
            req.low_stock_threshold.map(to_decimal),
        )
            .await
    }

    pub async fn get_one(pool: &sqlx::PgPool, id: Uuid, farm_id: Uuid) -> AppResult<RawMaterial> {
        RawMaterialRepository::find_by_id(pool, id, farm_id).await
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        id: Uuid,
        farm_id: Uuid,
        req: UpdateRawMaterialRequest,
    ) -> AppResult<RawMaterial> {
        let existing = RawMaterialRepository::find_by_id(pool, id, farm_id).await?;

        RawMaterialRepository::update(
            pool,
            id,
            farm_id,
            req.name.as_deref().unwrap_or(&existing.name),
            req.material_type.as_deref().unwrap_or(&existing.material_type),
            req.quantity.map(to_decimal).unwrap_or(existing.quantity),
            req.unit.as_deref().unwrap_or(&existing.unit),
            req.supplier.as_deref().or(existing.supplier.as_deref()),
            req.origin.as_deref().or(existing.origin.as_deref()),
            req.harvest_date.or(existing.harvest_date),
            req.expiry_date.or(existing.expiry_date),
            req.notes.as_deref().or(existing.notes.as_deref()),
            req.low_stock_threshold.map(to_decimal).or(existing.low_stock_threshold),
        )
            .await
    }

    pub async fn delete(pool: &sqlx::PgPool, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let rows = RawMaterialRepository::soft_delete(pool, id, farm_id).await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!("Raw material {} not found", id)));
        }
        Ok(())
    }

    pub async fn low_stock(pool: &sqlx::PgPool, farm_id: Uuid) -> AppResult<Vec<RawMaterial>> {
        RawMaterialRepository::find_low_stock(pool, farm_id).await
    }

    pub async fn adjust_quantity(
        pool: &sqlx::PgPool,
        id: Uuid,
        farm_id: Uuid,
        req: AdjustQuantityRequest,
    ) -> AppResult<RawMaterial> {
        RawMaterialRepository::adjust_quantity(pool, id, farm_id, to_decimal(req.delta))
            .await?
            .ok_or_else(|| AppError::BadRequest(
                "Material not found, not owned by your farm, or adjustment would result in negative stock".into(),
            ))
    }
}