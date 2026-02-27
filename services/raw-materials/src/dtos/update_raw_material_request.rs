use chrono::NaiveDate;
use serde::Deserialize;

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