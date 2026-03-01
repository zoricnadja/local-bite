use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct InsertRawMaterialParams {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub farm_id: Uuid,
    pub raw_material_id: Uuid,
    pub raw_material_name: String,
    pub material_type: String,
    pub quantity_used: BigDecimal,
    pub unit: String,
    pub origin: Option<String>,
    pub supplier: Option<String>,
}