use crate::common::query_params::{
    pagination::HasPagination,
    sorting::HasSorting, 
    filter::HasTextbookFilter
};

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct TextbookQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,

    pub sort_field: Option<String>,
    pub sort_order: Option<String>,

    pub level: Option<String>,
    pub is_active: Option<bool>,
}


impl HasPagination for TextbookQuery {
    fn page(&self) -> Option<i64> {
        self.page
    }

    fn limit(&self) -> Option<i64> {
        self.limit
    }
}


impl HasSorting for TextbookQuery {
    fn sort_field(&self) -> Option<&str> {
        self.sort_field.as_deref()
    }

    fn sort_oreder(&self) -> Option<&str> {
        self.sort_order.as_deref()
    }
}


impl HasTextbookFilter for TextbookQuery {
    fn level(&self) -> Option<&str> {
        self.level.as_deref()
    }
    
    fn is_active(&self) -> Option<bool> {
        self.is_active
    }
}