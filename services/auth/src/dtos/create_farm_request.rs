use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateFarmRequest {
    pub name: String,
}