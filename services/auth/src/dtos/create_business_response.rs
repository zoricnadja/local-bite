use crate::models::businesses::Business;

#[derive(serde::Serialize)]
pub struct CreateBusinessResult {
    pub business: Business,
    pub token: String,
}
