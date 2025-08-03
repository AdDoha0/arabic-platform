use async_trait::async_trait;
use crate::modules::textbooks::{    
    dto::{input::{CreateTextbookDto, UpdateTextbookDto}, output::TextbookResponseDto},
    entity::NewTextbook,
    query::TextbookQuery,
    repository::{self, repository_trait::TextbookRepository}
};

use super::service_trait::TextbookService;

use crate::common::{
    error::AppError, 
    response::PaginatedResponse
};

use crate::common::query_params::{
    pagination::HasPagination,
};


#[derive(Clone)]
pub struct TextbookServiceImpl<R>
where
    R: TextbookRepository,
{
    repository: R,
}


impl<R> TextbookServiceImpl<R>
where
    R: TextbookRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> TextbookService for TextbookServiceImpl<R>
where
    R: TextbookRepository + Send + Sync,
{ 

    async fn get_textbooks(&self, params: &TextbookQuery) -> Result<PaginatedResponse<TextbookResponseDto>, AppError> {
        let total =   self.repository.count_textbooks().await?;
        let textbooks = self.repository.select_all_textbooks(params).await?;

        let dto = textbooks.into_iter().map(Into::into).collect();

        Ok(PaginatedResponse::new(dto, total, params.page_or_default(), params.limit_or_default()))  
    }

    async fn get_textbook_by_id(&self, id: i32) -> Result<TextbookResponseDto, AppError> {
        let textbook = self.repository.select_textbook_by_id(id).await?;
        let textbook = textbook.ok_or(AppError::NotFound(format!("Textbook with id={} not found", id)))?;
        Ok(textbook.into())
    }

    async fn create_textbook(&self, dto: CreateTextbookDto) -> Result<TextbookResponseDto, AppError> {
        let new_tb = NewTextbook::from(dto);
        let textbook = self.repository.insert_textbook(
            new_tb
        )
        .await?;

        Ok(textbook.into())
    }

    async fn update_textbook(&self, id: i32, dto: UpdateTextbookDto) -> Result<TextbookResponseDto, AppError> {
        let updated = self.repository.update_textbook_by_id(id, dto).await?;
        Ok(updated.into())

    }

    async fn delete_textbook(&self, id: i32) -> Result<(), AppError> {
        let rows_affected = self.repository.delete_textbook_by_id(id).await?;
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Textbook with id={} not found", id)));
        }

        Ok(())
    }
}