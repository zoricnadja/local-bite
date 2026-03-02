use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}