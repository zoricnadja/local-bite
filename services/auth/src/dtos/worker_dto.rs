use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct WorkerOut {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub farm_id: Uuid,
}