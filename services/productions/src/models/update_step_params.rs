use bigdecimal::BigDecimal;

pub struct UpdateStepParams {
    pub step_order: i32,
    pub name: String,
    pub description: Option<String>,
    pub duration_hours: Option<BigDecimal>,
    pub temperature: Option<BigDecimal>,
}