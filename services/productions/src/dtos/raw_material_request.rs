use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct RawMaterialRequest {
    pub raw_material_id:   Uuid,
    pub quantity_used:     f64,
    pub unit:              String,
}