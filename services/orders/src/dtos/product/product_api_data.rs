use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ProductApiData {
    pub(crate) id:           Uuid,
    pub(crate) name:         String,
    pub(crate) product_type: String,
    pub(crate) price:        f64,
    pub(crate) unit:         String,
    pub(crate) is_active:    bool,
}
