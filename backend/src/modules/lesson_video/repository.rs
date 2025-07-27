use sqlx::PgPool;
use crate::common::error::AppError;
use super::entity::{LessonVideo, NewLessonVideo};
use super::dto::input::UpdateLessonVideoDto;

pub async fn insert_lesson_video(
    db: &PgPool,
    dto: NewLessonVideo,
) -> Result<LessonVideo, AppError> {
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
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(result)
}


pub async fn update_lesson_video(
    db: &PgPool,
    lesson_id: i32,
    dto: UpdateLessonVideoDto,
) -> Result<LessonVideo, AppError> {
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
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;
    result.ok_or_else(|| AppError::NotFound(format!("Video for lesson_id={} not found", lesson_id)))
}


pub async fn select_lesson_video_by_lesson_id(
    db: &PgPool,
    lesson_id: i32
) -> Result<Option<LessonVideo>, AppError> {
    let result = sqlx::query_as!(
        LessonVideo,
        r#"
        SELECT id, lesson_id, title, youtube_url
        FROM lessons_videos
        WHERE lesson_id = $1
        "#,
        lesson_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(result)
}


pub async fn delete_lesson_video_by_lesson_id(
    db: &PgPool,
    lesson_id: i32
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        "DELETE FROM lessons_videos WHERE lesson_id = $1",
        lesson_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(result.rows_affected())
} 