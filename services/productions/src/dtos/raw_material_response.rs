use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RawMaterialResponse {
    pub id: Uuid,
    pub name: String,
    pub material_type: String,
    pub quantity_used: f64,
    pub unit: String,
    pub origin: Option<String>,
    pub harvest_date: Option<chrono::NaiveDate>,
    pub received_date: Option<chrono::NaiveDate>,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub supplier: Option<String>,
}
