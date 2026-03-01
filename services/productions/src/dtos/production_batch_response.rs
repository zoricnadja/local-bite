use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;
use crate::dtos::process_step_response::ProcessStepResponse;
use crate::dtos::raw_material_response::RawMaterialResponse;

#[derive(Debug, Serialize)]
pub struct ProductionBatchResponse {
    pub id:            Uuid,
    pub farm_id:       Uuid,
    pub name:          String,
    pub process_type:  String,
    pub start_date:    Option<String>,   // ISO string for easy JSON
    pub end_date:      Option<String>,
    pub status:        String,
    pub notes:         Option<String>,
    pub steps:         Vec<ProcessStepResponse>,
    pub raw_materials: Vec<RawMaterialResponse>,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
}