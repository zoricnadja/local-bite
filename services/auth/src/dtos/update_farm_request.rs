use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateFarmRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub photo_url: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
}