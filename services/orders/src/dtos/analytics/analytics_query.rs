use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub from: Option<String>,   // YYYY-MM-DD
    pub to:   Option<String>,
}