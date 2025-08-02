use async_trait::async_trait;
use sqlx::PgPool;
use crate::common::error::AppError;
use crate::modules::lesson_video::entity::{LessonVideo, NewLessonVideo};
use crate::modules::lesson_video::dto::input::UpdateLessonVideoDto;
use crate::modules::lesson_video::repository::repository_trait::LessonVideoRepository;


#[derive(Clone)]
pub struct PostgresLessonVideoRepository {
    pool: PgPool,
}

impl PostgresLessonVideoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl LessonVideoRepository for PostgresLessonVideoRepository {
    async fn insert(&self, dto: NewLessonVideo) -> Result<LessonVideo, AppError> {
        let result = sqlx::query_as!(
            LessonVideo,
            r#"
            INSERT INTO lessons_videos (lesson_id, title, youtube_url)
            VALUES ($1, $2, $3)
            RETURNING id, lesson_id, title, youtube_url
            "#,
            dto.lesson_id,
            dto.title,
            dto.youtube_url
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    async fn update(&self, lesson_id: i32, dto: UpdateLessonVideoDto) -> Result<LessonVideo, AppError> {
        let result = sqlx::query_as!(
            LessonVideo,
            r#"
            UPDATE lessons_videos SET
                title = COALESCE($1, title),
                youtube_url = COALESCE($2, youtube_url)
            WHERE lesson_id = $3
            RETURNING id, lesson_id, title, youtube_url
            "#,
            dto.title,
            dto.youtube_url,
            lesson_id
        )
        .fetch_optional(&self.pool)
        .await?;
        result.ok_or_else(|| AppError::NotFound(format!("Video for lesson_id={} not found", lesson_id)))
    }

    async fn find_by_lesson_id(&self, lesson_id: i32) -> Result<Option<LessonVideo>, AppError> {
        let result = sqlx::query_as!(
            LessonVideo,
            r#"
            SELECT id, lesson_id, title, youtube_url
            FROM lessons_videos
            WHERE lesson_id = $1
            "#,
            lesson_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn delete_by_lesson_id(&self, lesson_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM lessons_videos WHERE lesson_id = $1",
            lesson_id
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}
