use chrono::NaiveDate;
use serde::Deserialize;
use crate::dtos::raw_material_request::RawMaterialRequest;

#[derive(Debug, Deserialize)]
pub struct CreateProductionBatchRequest {
    pub name:          String,
    pub process_type:  String,
    pub start_date:    Option<NaiveDate>,
    pub end_date:      Option<NaiveDate>,
    pub notes:         Option<String>,
    /// Raw materials to link at creation time (optional)
    pub raw_materials: Option<Vec<RawMaterialRequest>>,
}