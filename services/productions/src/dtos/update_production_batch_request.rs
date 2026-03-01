use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProductionBatchRequest {
    pub name:         Option<String>,
    pub process_type: Option<String>,
    pub start_date:   Option<NaiveDate>,
    pub end_date:     Option<NaiveDate>,
    pub notes:        Option<String>,
    pub status:       Option<String>,
}