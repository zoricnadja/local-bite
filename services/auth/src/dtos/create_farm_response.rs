use crate::models::farms::Farm;

#[derive(serde::Serialize)]
pub struct CreateFarmResult {
    pub farm: Farm,
    pub token: String,
}