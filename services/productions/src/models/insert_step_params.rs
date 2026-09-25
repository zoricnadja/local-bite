use uuid::Uuid;

pub struct InsertStepParams {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub business_id: Uuid,
    pub step_order: i32,
    pub name: String,
    pub description: Option<String>,
    pub variables: serde_json::Value,
}
