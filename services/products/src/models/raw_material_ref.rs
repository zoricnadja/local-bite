use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawMaterialRef {
    pub id: Uuid,
    pub name: String,
    pub material_type: String,
    pub quantity_used: f64,
    pub unit: String,
    pub origin: Option<String>,
    pub harvest_date: Option<chrono::NaiveDate>,
    pub received_date: Option<String>,
    pub expiry_date: Option<String>,
    pub supplier: Option<String>,
}
