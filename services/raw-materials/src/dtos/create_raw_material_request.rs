use chrono::NaiveDate;
use serde::Deserialize;

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