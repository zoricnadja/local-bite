use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateBusinessRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
}
