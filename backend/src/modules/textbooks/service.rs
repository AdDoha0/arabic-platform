use sqlx::PgPool;

use crate::common::query_params::pagination::HasPagination;
use crate::modules::textbooks::entity::NewTextbook;
use crate::modules::textbooks::dto::input::{CreateTextbookDto, UpdateTextbookDto};
use crate::modules::textbooks::dto::output::TextbookResponseDto;
use crate::modules::textbooks::repository;

use crate::common::error::AppError;
use super::query::TextbookQuery;
use crate::common::response::PaginatedResponse;


// Response (Ответ) Request (Запрос)

pub async fn create_textbook(
    db: &PgPool, 
    dto: CreateTextbookDto,
) -> Result<TextbookResponseDto, AppError> {
    let new_tb = NewTextbook::from(dto);

    let textbook = repository::insert_textbook(
        db,
        new_tb.title,
        new_tb.description,
        new_tb.level,
        new_tb.is_active,
    )
    .await?;

    Ok(textbook.into())
}


pub async fn get_textbook_by_id(
    db: &PgPool,
    id: i32
) -> Result<TextbookResponseDto, AppError> {
    let textbook = repository::select_textbook_by_id(db, id).await?;

    let textbook = textbook.ok_or(AppError::NotFound(format!("Textbook with id={} not found", id)))?;

    Ok(textbook.into())
}


pub async fn list_textbooks(
    db: &PgPool,
    pagination: TextbookQuery,
) -> Result<PaginatedResponse<TextbookResponseDto>, AppError> {
    let total = repository::count_textbooks(db).await?;
    let textbooks = repository::select_all_textbooks(db, &pagination).await?;

    let dto = textbooks.into_iter().map(Into::into).collect();

    Ok(PaginatedResponse::new(dto, total, pagination.page_or_default(), pagination.limit_or_default()))
}


pub async fn patch_textbook(
    db: &PgPool,
    id: i32,
    dto: UpdateTextbookDto,
) -> Result<TextbookResponseDto, AppError> {
    let updated = repository::update_textbook_by_id(db, id, dto).await?;
    Ok(updated.into())
}


pub async fn delete_textbook(
    db: &PgPool,
    id: i32,
) -> Result<(), AppError> {
    let rows_affected = repository::delete_textbook_by_id(db, id).await?;

    if rows_affected == 0 {
        return Err(AppError::NotFound(format!("Textbook with id={} not found", id)));
    }

    Ok(())
}

