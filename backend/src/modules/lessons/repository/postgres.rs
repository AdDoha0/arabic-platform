use sqlx::{QueryBuilder, Postgres, PgPool};

use crate::common::query_params::{
    pagination::HasPagination,
    sorting::HasSorting,
};

use async_trait::async_trait;
use crate::common::error::AppError;
use crate::modules::lessons::entity::{Lesson, NewLesson};
use crate::modules::lessons::dto::input::UpdateLessonDto;
use crate::modules::lessons::query::LessonQuery;

use super::repository_trait::LessonRepository;


#[derive(Clone)]
pub struct PostgresLessonRepository {
    pool: PgPool,
}

impl PostgresLessonRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}



#[async_trait]
impl LessonRepository for PostgresLessonRepository {
    async fn select_all_lessons(&self, params: &LessonQuery) -> Result<Vec<Lesson>, AppError> {
       let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT id, title, description, textbook_id, created_at FROM lessons"
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
            .fetch_all(&self.pool)
            .await?;
                
        Ok(result)
    }

    async fn count_lessons(&self) -> Result<i64, AppError> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM lessons")
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0);

        Ok(count)
    }

    async fn insert_lesson(&self, dto: NewLesson) -> Result<Lesson, AppError> {
        let result = sqlx::query_as!(
            Lesson,
            r#"
            INSERT INTO lessons (textbook_id, title, description)
            VALUES ($1, $2, $3)
            RETURNING id, textbook_id, title, description, created_at
            "#,
            dto.textbook_id, 
            dto.title,
            dto.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn select_lesson_by_id(&self, id: i32) -> Result<Option<Lesson>, AppError> {
        let result = sqlx::query_as!(
            Lesson,
            r#"
            SELECT id, textbook_id, title, description, created_at
                FROM lessons
            WHERE id = $1 
            "#,
            id        
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete_lesson_by_id(&self, id: i32) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM lessons WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn update_lesson_by_id(&self, id: i32, dto: UpdateLessonDto) -> Result<Lesson, AppError> {
        let result = sqlx::query_as!(
            Lesson, 
            r#"
            UPDATE lessons SET
                textbook_id = COALESCE($1, textbook_id),
                title = COALESCE($2, title),
                description = COALESCE($3, description)
            WHERE id = $4
            RETURNING id, textbook_id, title, description, created_at
            "#, 
            dto.textbook_id,
            dto.title, 
            dto.description,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        result.ok_or_else(|| AppError::NotFound(format!("Lesson with id={} not found", id)))
    }
}



// // GET /lessons — список всех уроков (с базовыми фильтрами/пагинацией/сортировкой по желанию)


// pub async fn select_all_lessons(
//     db: &PgPool,
//     params: &LessonQuery,
// ) -> Result<Vec<Lesson>, AppError> {
//     let mut builder = QueryBuilder::<Postgres>::new(
//         "SELECT id, title, description, textbook_id, created_at FROM lessons"
//     );

//     let sort_field: &'static str = match params.sort_field() {
//         Some("title") => "title",
//         _ => "created_at"
//     };

//     let sort_order = match params.sort_oreder() {
//         Some("asc") => "ASC",
//         _ => "DESC"       
//     };

//     builder
//         .push(" ORDER BY ")
//         .push(sort_field)
//         .push(" ")
//         .push(sort_order);


//     builder
//         .push(" LIMIT ")
//         .push_bind(params.limit_or_default())
//         .push(" OFFSET ")
//         .push_bind(params.offset());

//         let query = builder.build_query_as::<Lesson>();

//         let result = query
//         .fetch_all(db)
//         .await?;
    
//     Ok(result)
// }


// pub async fn count_lessons(db: &PgPool) -> Result<i64, AppError> {
//     let count = sqlx::query_scalar!("SELECT COUNT(*) FROM lessons")
//         .fetch_one(db)
//         .await?
//         .unwrap_or(0);

//     Ok(count)
// }

// pub async fn insert_lesson(
//     db: &PgPool,
//     dto: NewLesson
// ) -> Result<Lesson, AppError> {
//     let result = sqlx::query_as!(
//         Lesson,
//         r#"
//         INSERT INTO lessons (textbook_id, title, description)
//         VALUES ($1, $2, $3)
//         RETURNING id, textbook_id, title, description, created_at
//         "#,
//         dto.textbook_id, 
//         dto.title,
//         dto.description
//     )
//     .fetch_one(db)
//     .await?;

//     Ok(result)
// }



// pub async fn select_lesson_by_id(
//     db: &PgPool, 
//     id: i32
// ) -> Result<Option<Lesson>, AppError> {
//     let result = sqlx::query_as!(
//         Lesson,
//         r#"
//         SELECT id, textbook_id, title, description, created_at
//             FROM lessons
//         WHERE id = $1 
//         "#,
//         id        
//     )
//     .fetch_optional(db)
//     .await?;

//     Ok(result)
// }



// pub async fn delete_lesson_by_id(
//     db: &PgPool,
//     id: i32
// ) -> Result<u64, AppError> {
//     let result = sqlx::query!(
//         "DELETE FROM lessons WHERE id = $1",
//         id
//     )
//     .execute(db)
//     .await?;

//     Ok(result.rows_affected())

// }


// pub async fn update_lesson_by_id(
//     db: &PgPool,
//     id: i32,
//     dto: UpdateLessonDto
// ) ->  Result<Lesson, AppError> {
//     let result = sqlx::query_as!(
//         Lesson, 
//         r#"
//         UPDATE lessons SET
//             textbook_id = COALESCE($1, textbook_id),
//             title = COALESCE($2, title),
//             description = COALESCE($3, description)
//         WHERE id = $4
//         RETURNING id, textbook_id, title, description, created_at
//         "#, 
//         dto.textbook_id,
//         dto.title, 
//         dto.description,
//         id
//     )
//     .fetch_optional(db)
//     .await?;

//     result.ok_or_else(|| AppError::NotFound(format!("Lesson with id={} not found", id)))

// }