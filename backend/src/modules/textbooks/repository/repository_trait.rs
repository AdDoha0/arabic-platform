use async_trait::async_trait;
use crate::common::error::AppError;

use crate::modules::textbooks::entity::{Textbook, NewTextbook };
use crate::modules::textbooks::dto::input::UpdateTextbookDto;


use crate::modules::textbooks::query::TextbookQuery;


#[async_trait]
pub trait TextbookRepository: Send + Sync {
    async fn select_all_textbooks(&self, params: &TextbookQuery) -> Result<Vec<Textbook>, AppError>;
    async fn count_textbooks(&self) -> Result<i64, AppError>;
    async fn insert_textbook(&self, dto: NewTextbook) -> Result<Textbook, AppError>;
    async fn select_textbook_by_id(&self, id: i32) -> Result<Option<Textbook>, AppError>;
    async fn delete_textbook_by_id(&self, id: i32) -> Result<u64, AppError>;
    async fn update_textbook_by_id(&self, id: i32, dto: UpdateTextbookDto) -> Result<Textbook, AppError>;
}