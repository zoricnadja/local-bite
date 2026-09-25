use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::adjust_quantity_request::AdjustQuantityRequest;
use crate::dtos::create_raw_material_request::CreateRawMaterialRequest;
use crate::dtos::update_raw_material_request::UpdateRawMaterialRequest;
use crate::models::query::ListQuery;
use crate::models::raw_material::RawMaterial;
use crate::repository::repository::RawMaterialRepository;
use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;

fn to_decimal(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_default()
}

#[derive(Clone)]
pub struct RawMaterialService {
    pub raw_material_repository: Arc<RawMaterialRepository>,
}
impl RawMaterialService {
    pub fn new(raw_material_repository: Arc<RawMaterialRepository>) -> Self {
        Self {
            raw_material_repository,
        }
    }
    pub async fn list(
        &self,
        business_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<PaginatedResponse<RawMaterial>> {
        let (data, total) = self.raw_material_repository.find_all(business_id, q).await?;
        Ok(PaginatedResponse {
            data,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    pub async fn create(
        &self,
        business_id: Uuid,
        req: CreateRawMaterialRequest,
    ) -> AppResult<RawMaterial> {
        validate_dates(req.harvest_date, req.received_date, req.expiry_date)?;
        if req.name.trim().is_empty() {
            return Err(AppError::BadRequest("Name cannot be empty".into()));
        }
        if req.quantity < 0.0 {
            return Err(AppError::BadRequest("Quantity cannot be negative".into()));
        }
        if req.unit.trim().is_empty() {
            return Err(AppError::BadRequest("Unit cannot be empty".into()));
        }

        self.raw_material_repository
            .insert(
                Uuid::new_v4(),
                business_id,
                req.name.trim(),
                req.material_type.trim(),
                to_decimal(req.quantity),
                req.unit.trim(),
                req.supplier.as_deref(),
                req.origin.as_deref(),
                req.received_date,
                req.harvest_date,
                req.expiry_date,
                req.notes.as_deref(),
                req.low_stock_threshold.map(to_decimal),
            )
            .await
    }

    pub async fn get_one(&self, id: Uuid, business_id: Uuid) -> AppResult<RawMaterial> {
        self.raw_material_repository.find_by_id(id, business_id).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        business_id: Uuid,
        req: UpdateRawMaterialRequest,
    ) -> AppResult<RawMaterial> {
        let existing = self.raw_material_repository.find_by_id(id, business_id).await?;
        validate_dates(req.harvest_date.or(existing.harvest_date), req.received_date.or(existing.received_date), req.expiry_date.or(existing.expiry_date))?;

        self.raw_material_repository
            .update(
                id,
                business_id,
                req.name.as_deref().unwrap_or(&existing.name),
                req.material_type
                    .as_deref()
                    .unwrap_or(&existing.material_type),
                req.quantity.map(to_decimal).unwrap_or(existing.quantity),
                req.unit.as_deref().unwrap_or(&existing.unit),
                req.supplier.as_deref().or(existing.supplier.as_deref()),
                req.origin.as_deref().or(existing.origin.as_deref()),
                req.received_date.or(existing.received_date),
                req.harvest_date.or(existing.harvest_date),
                req.expiry_date.or(existing.expiry_date),
                req.notes.as_deref().or(existing.notes.as_deref()),
                req.low_stock_threshold
                    .map(to_decimal)
                    .or(existing.low_stock_threshold),
            )
            .await
    }

    pub async fn delete(&self, id: Uuid, business_id: Uuid) -> AppResult<()> {
        let rows = self
            .raw_material_repository
            .soft_delete(id, business_id)
            .await?;
        if rows == 0 {
            return Err(AppError::NotFound(format!("Raw material {} not found", id)));
        }
        Ok(())
    }

    pub async fn low_stock(&self, business_id: Uuid) -> AppResult<Vec<RawMaterial>> {
        self.raw_material_repository.find_low_stock(business_id).await
    }

    pub async fn adjust_quantity(
        &self,
        id: Uuid,
        business_id: Uuid,
        req: AdjustQuantityRequest,
    ) -> AppResult<RawMaterial> {
        self.raw_material_repository.adjust_quantity(id, business_id, to_decimal(req.delta))
            .await?
            .ok_or_else(|| AppError::BadRequest(
                "Material not found, not owned by your business, or adjustment would result in negative stock".into(),
            ))
    }
}

fn validate_dates(harvest: Option<chrono::NaiveDate>, received: Option<chrono::NaiveDate>, expiry: Option<chrono::NaiveDate>) -> AppResult<()> {
    if let Some(end) = expiry {
        if harvest.is_some_and(|start| start > end) || received.is_some_and(|start| start > end) {
            return Err(AppError::BadRequest("Expiry cannot be before harvest or receipt".into()));
        }
    }
    Ok(())
}
