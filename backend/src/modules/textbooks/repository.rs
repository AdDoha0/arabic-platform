use sqlx::{QueryBuilder, Postgres, PgPool};

use crate::common::query_params::{
    pagination::HasPagination,
    sorting::HasSorting,
    filter::HasTextbookFilter
};
use crate::modules::textbooks::entity::{NewTextbook, Textbook};
use crate::modules::textbooks::dto::input::UpdateTextbookDto;
use crate::common::error::AppError;
use super::query::TextbookQuery;


pub async fn insert_textbook(
    db: &PgPool,
    dto: NewTextbook
) -> Result<Textbook, AppError> {
    let result = sqlx::query_as!(
        Textbook,
        r#"
        INSERT INTO textbooks (title, description, level, is_active)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, description, level, is_active
        "#,
        dto.title,
        dto.description,
        dto.level,
        dto.is_active
    )
    .fetch_one(db)
    .await?;

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
    .await?;

    Ok(result)
}


pub async fn count_textbooks(db: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM textbooks")
        .fetch_one(db)
        .await
?
        .unwrap_or(0);

    Ok(count)
}



pub async fn select_all_textbooks(
    db: &PgPool,
    params: &TextbookQuery,
) -> Result<Vec<Textbook>, AppError> {
    let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT id, title, description, level, is_active FROM textbooks"
    );

    let mut has_where = false;


    if let Some(level) = params.level() {
        builder.push(if has_where { " AND" } else { " WHERE" });
        builder.push(" level = ").push_bind(level);
        has_where = true;
    }

    if let Some(is_active) = params.is_active() {
        builder.push(if has_where { " AND" } else { " WHERE" });
        builder.push(" is_active = ").push_bind(is_active);
        has_where = true;
    }

    let sort_field: &'static str = match params.sort_field() {
        Some("title") => "title",
        Some("level") => "level",
        _ => "id"
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
    

    let query = builder.build_query_as::<Textbook>();

    let result = query
        .fetch_all(db)
        .await?;

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
    .await?;

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
    .await?;

    Ok(result.rows_affected())
}


