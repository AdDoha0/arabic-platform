use sqlx::PgPool;

use crate::common::query_params::pagination::HasPagination;
use crate::modules::textbooks::entity::Textbook;
use crate::modules::textbooks::dto::input::UpdateTextbookDto;
use crate::common::error::AppError;
use super::query::TextbookQuery;


pub async fn insert_textbook(
    db: &PgPool,
    title: String,
    description: Option<String>,
    level: Option<String>,
    is_active: bool,
) -> Result<Textbook, AppError> {
    let result = sqlx::query_as!(
        Textbook,
        r#"
        INSERT INTO textbooks (title, description, level, is_active)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, description, level, is_active
        "#,
        title,
        description,
        level,
        is_active
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}


pub async fn select_textbook_by_id(
    db: &PgPool,
    id: i32
) -> Result<Option<Textbook>, AppError> {
    let result = sqlx::query_as!(
        Textbook, 
        r#"
        SELECT id, title, description, level, is_active
        FROM textbooks
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}


pub async fn count_textbooks(db: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM textbooks")
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .unwrap_or(0);

    Ok(count)
}


pub async fn select_all_textbooks(
    db: &PgPool,
    pagination: &TextbookQuery
) -> Result<Vec<Textbook>, AppError> {
    let result = sqlx::query_as!(
        Textbook,
        r#"
        SELECT id, title, description, level, is_active
        FROM textbooks
        ORDER BY id DESC
        LIMIT $1 OFFSET $2
        "#,
        pagination.limit_or_default(),
        pagination.offset()
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}


pub async fn update_textbook_by_id(
    db: &PgPool,
    id: i32,
    dto: UpdateTextbookDto
) -> Result<Textbook, AppError> {
    let result = sqlx::query_as!(
        Textbook,
        r#"
        UPDATE textbooks SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            level = COALESCE($3, level),
            is_active = COALESCE($4, is_active)
        WHERE id = $5
        RETURNING id, title, description, level, is_active
        "#,
        dto.title,
        dto.description,
        dto.level,
        dto.is_active,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    result.ok_or_else(|| AppError::NotFound(format!("Textbook with id={} not found", id)))
}


pub async fn delete_textbook_by_id(
    db: &PgPool,
    id: i32,
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM textbooks WHERE id = $1"#,
        id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result.rows_affected())
}


