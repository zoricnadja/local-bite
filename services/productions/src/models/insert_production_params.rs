use uuid::Uuid;

pub struct InsertProductionParams {
    pub id: Uuid,
    pub farm_id: Uuid,
    pub name: String,
    pub process_type: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
}