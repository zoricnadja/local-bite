use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProductionBatchRequest {
    pub name: Option<String>,
    pub process_type: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub output_name: Option<String>,
    pub output_type: Option<String>,
    pub output_unit: Option<String>,
    pub output_quantity: Option<f64>,
    pub output_expiry_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
}
