use async_trait::async_trait;

use crate::modules::textbooks::{    
    dto::input::{CreateTextbookDto, UpdateTextbookDto},
    dto::output::TextbookResponseDto,
    query::TextbookQuery,
};

use crate::common::error::AppError;
use crate::common::response::PaginatedResponse;


#[async_trait]
pub trait TextbookService: Send + Sync {
    async fn get_textbooks(&self, params: &TextbookQuery) -> Result<PaginatedResponse<TextbookResponseDto>, AppError>;
    async fn get_textbook_by_id(&self, id: i32) -> Result<TextbookResponseDto, AppError>;
    async fn create_textbook(&self, dto: CreateTextbookDto) -> Result<TextbookResponseDto, AppError>;
    async fn update_textbook(&self, id: i32, dto: UpdateTextbookDto) -> Result<TextbookResponseDto, AppError>;
    async fn delete_textbook(&self, id: i32) -> Result<(), AppError>;
}