use std::io::Seek;

use sqlx::{QueryBuilder, Postgres, PgPool};

use crate::common::query_params::{
    pagination::HasPagination,
    sorting::HasSorting,
    filter::HasTextbookFilter
};

use async_trait::async_trait;
use crate::common::error::AppError;
use crate::modules::textbooks::entity::{Textbook, NewTextbook};
use crate::modules::textbooks::dto::input::UpdateTextbookDto;
use crate::modules::textbooks::query::TextbookQuery;

use super::repository_trait::TextbookRepository;


#[derive(Clone)]
pub struct PostgresTextbookRepository {
    pool: PgPool,
}

impl PostgresTextbookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}



#[async_trait]
impl TextbookRepository for PostgresTextbookRepository {

    async fn select_all_textbooks(&self, params: &TextbookQuery) -> Result<Vec<Textbook>, AppError> {

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
            .fetch_all(&self.pool)
            .await?;

        Ok(result)

        }

        async fn count_textbooks(&self) -> Result<i64, AppError> {
            let count = sqlx::query_scalar!("SELECT COUNT(*) FROM textbooks")
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(0);

            Ok(count)
    }

    async fn insert_textbook(&self, dto: NewTextbook) -> Result<Textbook, AppError> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(result)

    }

    async fn select_textbook_by_id(&self, id: i32) -> Result<Option<Textbook>, AppError> {
        let result = sqlx::query_as!(
            Textbook, 
            r#"
            SELECT id, title, description, level, is_active
            FROM textbooks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)

    }

    async fn delete_textbook_by_id(&self, id: i32) -> Result<u64, AppError> {
        let result = sqlx::query!(
            r#"DELETE FROM textbooks WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())

    }

    async fn update_textbook_by_id(&self, id: i32, dto: UpdateTextbookDto) -> Result<Textbook, AppError> {
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
        .fetch_optional(&self.pool)
        .await?;

        result.ok_or_else(|| AppError::NotFound(format!("Textbook with id={} not found", id)))

    }
}



