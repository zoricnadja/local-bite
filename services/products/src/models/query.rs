use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page:         Option<i64>,
    pub limit:        Option<i64>,
    pub product_type: Option<String>,
    pub search:       Option<String>,
    pub active_only:  Option<bool>,
}

impl ListQuery {
    pub fn offset(&self) -> i64 {
        (self.page.unwrap_or(1).max(1) - 1) * self.limit()
    }
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
}