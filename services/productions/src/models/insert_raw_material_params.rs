use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct InsertRawMaterialParams {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub business_id: Uuid,
    pub raw_material_id: Uuid,
    pub raw_material_name: String,
    pub material_type: String,
    pub quantity_used: BigDecimal,
    pub unit: String,
    pub origin: Option<String>,
    pub harvest_date: Option<chrono::NaiveDate>,
    pub received_date: Option<chrono::NaiveDate>,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub supplier: Option<String>,
}
