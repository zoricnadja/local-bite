use serde::Serialize;
use crate::models::batch_ref::BatchRef;
use crate::models::product::Product;

#[derive(Debug, Serialize)]
pub struct ProvenanceResponse {
    pub product:    Product,
    pub farm_name:  Option<String>,
    pub batch:      Option<BatchRef>,
}