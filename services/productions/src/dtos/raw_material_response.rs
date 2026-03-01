use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RawMaterialResponse {
    pub id:            Uuid,
    pub name:          String,
    pub material_type: String,
    pub quantity_used: f64,
    pub unit:          String,
    pub origin:        Option<String>,
    pub supplier:      Option<String>,
}