use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub role: Option<String>,

    // Required
    pub first_name: String,
    pub last_name: String,
    pub address: String,

    // Optional
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}
