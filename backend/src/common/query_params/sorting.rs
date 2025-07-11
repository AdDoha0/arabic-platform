pub trait HasSorting {
    fn sort_field(&self) -> Option<&str>;
    fn sort_oreder(&self) -> Option<&str>;

    fn sort_order_or_default(&self) -> &str {
        self.sort_oreder().unwrap_or("asc")
    }
} 