use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── DB row ────────────────────────────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct RawMaterial {
    pub id:                  Uuid,
    pub farm_id:             Uuid,
    pub name:                String,
    pub material_type:       String,
    pub quantity:            BigDecimal,
    pub unit:                String,
    pub supplier:            Option<String>,
    pub origin:              Option<String>,
    pub harvest_date:        Option<NaiveDate>,
    pub expiry_date:         Option<NaiveDate>,
    pub notes:               Option<String>,
    pub low_stock_threshold: Option<BigDecimal>,
    pub is_deleted:          bool,
    pub created_at:          NaiveDateTime,
    pub updated_at:          NaiveDateTime,
}

// ── Request DTOs ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateRawMaterialRequest {
    pub name:                String,
    pub material_type:       String,
    pub quantity:            f64,
    pub unit:                String,
    pub supplier:            Option<String>,
    pub origin:              Option<String>,
    pub harvest_date:        Option<NaiveDate>,  // "YYYY-MM-DD"
    pub expiry_date:         Option<NaiveDate>,
    pub notes:               Option<String>,
    pub low_stock_threshold: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRawMaterialRequest {
    pub name:                Option<String>,
    pub material_type:       Option<String>,
    pub quantity:            Option<f64>,
    pub unit:                Option<String>,
    pub supplier:            Option<String>,
    pub origin:              Option<String>,
    pub harvest_date:        Option<NaiveDate>,
    pub expiry_date:         Option<NaiveDate>,
    pub notes:               Option<String>,
    pub low_stock_threshold: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustQuantityRequest {
    /// Positive = restock, negative = usage/deduction
    pub delta:  f64,
    pub reason: Option<String>,
}

// ── Query params ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page:          Option<i64>,
    pub limit:         Option<i64>,
    pub material_type: Option<String>,  // filter by type
    pub search:        Option<String>,  // search by name
}

impl ListQuery {
    pub fn offset(&self) -> i64 {
        let page  = self.page.unwrap_or(1).max(1);
        let limit = self.limit.unwrap_or(20).min(100);
        (page - 1) * limit
    }
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
}

// ── Response DTOs ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data:  Vec<T>,
    pub total: i64,
    pub page:  i64,
    pub limit: i64,
}
