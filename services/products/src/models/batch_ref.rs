use crate::models::process_step_ref::ProcessStepRef;
use crate::models::raw_material_ref::RawMaterialRef;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchRef {
    pub id: Uuid,
    pub name: String,
    pub process_type: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: String,
    pub steps: Vec<ProcessStepRef>,
    pub raw_materials: Vec<RawMaterialRef>,
}
