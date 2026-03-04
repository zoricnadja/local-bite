use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>, // ADMIN only

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}