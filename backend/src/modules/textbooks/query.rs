use crate::common::pagination::HasPagination;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct TextbookQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub level: Option<String>, // Фильтрация (если нужно)
}

impl HasPagination for TextbookQuery {
    fn page(&self) -> Option<i64> {
        self.page
    }

    fn limit(&self) -> Option<i64> {
        self.limit
    }
}