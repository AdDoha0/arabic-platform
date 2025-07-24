use crate::common::query_params::{
    pagination::HasPagination,
    sorting::HasSorting, 
    filter::HasTextbookFilter
};

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct LessonQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,

    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
}


impl HasPagination for LessonQuery {
    fn page(&self) -> Option<i64> {
        self.page
    }

    fn limit(&self) -> Option<i64> {
        self.limit
    }
}


impl HasSorting for LessonQuery {
    fn sort_field(&self) -> Option<&str> {
        self.sort_field.as_deref()
    }

    fn sort_oreder(&self) -> Option<&str> {
        self.sort_order.as_deref()
    }
}

