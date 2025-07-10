use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,      
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(10)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }
}