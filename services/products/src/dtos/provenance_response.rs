use crate::models::batch_ref::BatchRef;
use crate::models::product::Product;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProvenanceResponse {
    pub product: Product,
    pub business_name: Option<String>,
    pub batch: Option<BatchRef>,
}
