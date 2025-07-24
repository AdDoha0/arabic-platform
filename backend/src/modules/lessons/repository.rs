use std::result;

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
use super::query::LessonQuery;



// GET /lessons — список всех уроков (с базовыми фильтрами/пагинацией/сортировкой по желанию)


pub async fn select_all_lessons(
    db: &PgPool,
    params: &LessonQuery,
) -> Result<Vec<Lesson>, AppError> {
    let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT id, title, description, textbook_id, created_at FROM textbooks"
    );

    let sort_field: &'static str = match params.sort_field() {
        Some("title") => "title",
        _ => "created_at"
    };

    let sort_order = match params.sort_oreder() {
        Some("asc") => "ASC",
        _ => "DESC"       
    };

    builder
    .push(" ORDER BY ")
    .push(sort_field)
    .push(" ")
    .push(sort_order);


builder
    .push(" LIMIT ")
    .push_bind(params.limit_or_default())
    .push(" OFFSET ")
    .push_bind(params.offset());

    let query = builder.build_query_as::<Lesson>();

    let result = query
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

Ok(result)


}




pub async fn insert_lesson(
    db: &PgPool,
    textbook_id: i32, 
    title: Option<String>,
    description: Option<String>
) -> Result<Lesson, AppError> {
    let result = sqlx::query_as!(
        r#"
        INSERT INTO lessons (textbook_id, title, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, textbook_id, title, description, created_at
        "#,
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}



pub async fn select_lesson_by_id(
    db: &PgPool, 
    id: i32
) -> Result<Option<Lesson>, AppError> {
    let result = sqlx::query_as!(
        Lesson,
        r#"
        SELECT id, textbook_id, title, description, created_at
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



pub async fn delete_lesson_by_id(
    db: &PgPool,
    id: i32
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        "DELETE FROM lessons WHERE id = $1",
        id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result.rows_affected())

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