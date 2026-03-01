use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct InsertStepParams {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub farm_id: Uuid,
    pub step_order: i32,
    pub name: String,
    pub description: Option<String>,
    pub duration_hours: Option<BigDecimal>,
    pub temperature: Option<BigDecimal>,
}