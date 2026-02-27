use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page:          Option<i64>,
    pub limit:         Option<i64>,
    pub material_type: Option<String>,  // filter by type
    pub search:        Option<String>,  // search by name
}

impl ListQuery {
    pub fn offset(&self) -> i64 {
        let page  = self.page.unwrap_or(1).max(1);
        let limit = self.limit.unwrap_or(20).min(100);
        (page - 1) * limit
    }
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
}
