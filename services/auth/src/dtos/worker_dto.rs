use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct WorkerOut {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub business_id: Uuid,
}
