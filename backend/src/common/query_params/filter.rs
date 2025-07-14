pub trait HasTextbookFilter {
    fn level(&self) -> Option<&str>;
    fn is_active(&self) -> Option<bool>;
}
