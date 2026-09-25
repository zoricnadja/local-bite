use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateBusinessRequest {
    pub name: String,

    // Required
    pub address: String,

    // Optional
    pub phone: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
}
