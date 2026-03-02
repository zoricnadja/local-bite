use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListOrdersQuery {
    pub page:     Option<i64>,
    pub limit:    Option<i64>,
    pub status:   Option<String>,
    pub search:   Option<String>,   // searches customer name/email
}

impl ListOrdersQuery {
    pub fn offset(&self) -> i64 {
        (self.page.unwrap_or(1).max(1) - 1) * self.limit()
    }
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
}