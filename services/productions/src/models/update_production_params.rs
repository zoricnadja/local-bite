pub struct UpdateProductionParams {
    pub name: String,
    pub process_type: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
    pub status: String,
}