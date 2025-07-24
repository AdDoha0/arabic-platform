use sqlx::PgPool;

use super::{    
    dto::input::{CreateLessonDto, UpdateLessonDto},
    dto::output::LessonResponseDto,
    entity::NewLesson,
    query::LessonQuery,
    repository
};

use crate::common::{
    error::AppError, query_params::pagination::HasPagination, response::PaginatedResponse
};



pub async fn create_lesson(
    db: &PgPool, 
    dto: CreateLessonDto,
) -> Result<LessonResponseDto, AppError> {
    let new_tb = NewLesson::from(dto);

    let lesson = repository::insert_lesson(
        db,
        new_tb.textbook_id,
        new_tb.title,
        new_tb.description
    )
    .await?;

    Ok(lesson.into())
}



pub async fn get_lesson_by_id(
    db: &PgPool,
    id: i32
) -> Result<LessonResponseDto, AppError> {
    let lesson = repository::select_lesson_by_id(db, id).await?;

    let lesson = lesson.ok_or(AppError::NotFound(format!("Lesson with id={} not found", id)))?;

    Ok(lesson.into())
}


pub async fn list_lessons(
    db: &PgPool,
    pagination: LessonQuery,
) -> Result<PaginatedResponse<LessonResponseDto>, AppError> {
    let total = repository::count_lessons(db).await?;
    let lessons = repository::select_all_lessons(db, &pagination).await?;

    let dto = lessons.into_iter().map(Into::into).collect();

    Ok(PaginatedResponse::new(dto, total, pagination.page_or_default(), pagination.limit_or_default()))
}


pub async fn patch_lesson(
    db: &PgPool,
    id: i32,
    dto: UpdateLessonDto,
) -> Result<LessonResponseDto, AppError> {
    let updated = repository::update_lesson_by_id(db, id, dto).await?;
    Ok(updated.into())
}


pub async fn delete_lesson(
    db: &PgPool,
    id: i32,
) -> Result<(), AppError> {
    let rows_affected = repository::delete_lesson_by_id(db, id).await?;

    if rows_affected == 0 {
        return Err(AppError::NotFound(format!("Lesson with id={} not found", id)));
    }

    Ok(())
}
