pub struct UpdateProductionParams {
    pub name: String,
    pub process_type: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
    pub outputs: serde_json::Value,
    pub output_name: Option<String>,
    pub output_type: Option<String>,
    pub output_unit: Option<String>,
    pub output_quantity: Option<f64>,
    pub output_expiry_date: Option<chrono::NaiveDate>,
    pub status: String,
}
