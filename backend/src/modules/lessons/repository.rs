use sqlx::{QueryBuilder, Postgres, PgPool};

use crate::common::query_params::{
    pagination::HasPagination,
    sorting::HasSorting,
    filter::HasTextbookFilter
};

use crate::modules::lessons::{
    dto::input::{CreateLessonDto, UpdateLessonDto},
    entity::Lesson
};

use crate::common::error::AppError;


pub async fn select_lesson_by_id(
    db: &PgPool, 
    id: i32
) -> Result<Option<Lesson>, AppError> {
    let result = sqlx::query_as!(
        Lesson,
        r#"
        SELECT id, textbook_id, title, description
            FROM lessons
        WHERE id = $1 
        "#,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}



pub async fn update_lesson_by_id(
    db: &PgPool,
    id: i32,
    dto: UpdateLessonDto
) ->  Result<Lesson, AppError> {
    let result = sqlx::query_as!(
        Lesson, 
        r#"
        UPDATE lessons SET
            textbook_id = COALESCE($1, textbook_id),
            title = COALESCE($2, title),
            description = COALESCE($3, description)
        WHERE id = $4
        RETURNING id, textbook_id, title, description
        "#, 
        dto.textbook_id,
        dto.title, 
        dto.description,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    result.ok_or_else(|| AppError::NotFound(format!("Lesson with id={} not found", id)))

}