use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddWorkerRequest {
    pub email: String,
    pub password: String,
}