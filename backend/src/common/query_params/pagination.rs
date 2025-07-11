pub trait HasPagination{
    fn page(&self) -> Option<i64>;
    fn limit(&self) -> Option<i64>;

    fn page_or_default(&self) -> i64 {
        self.page().unwrap_or(1)
    }

    fn limit_or_default(&self) -> i64 {
        self.limit().unwrap_or(10).min(100)
    }

    fn offset(&self) -> i64 {
        (self.page_or_default() - 1) * self.limit_or_default()
    }
}