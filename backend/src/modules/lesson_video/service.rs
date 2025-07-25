use sqlx::PgPool;
use crate::common::error::AppError;
use super::repository;
use super::entity::{NewLessonVideo};
use super::dto::input::{CreateLessonVideoDto, UpdateLessonVideoDto};
use super::dto::output::LessonVideoResponseDto;


pub async fn get_lesson_video(
    db: &PgPool,
    lesson_id: i32
) -> Result<LessonVideoResponseDto, AppError> {
    let video = repository::select_lesson_video_by_lesson_id(db, lesson_id).await?;
    let video = video.ok_or(AppError::NotFound(format!("Video for lesson_id={} not found", lesson_id)))?;
    Ok(video.into())
}


pub async fn create_lesson_video(
    db: &PgPool,
    dto: CreateLessonVideoDto
) -> Result<LessonVideoResponseDto, AppError> {
    let new_video = NewLessonVideo::from(dto);
    let video = repository::insert_lesson_video(db, new_video).await?;
    Ok(video.into())
}


pub async fn update_lesson_video(
    db: &PgPool,
    lesson_id: i32,
    dto: UpdateLessonVideoDto
) -> Result<LessonVideoResponseDto, AppError> {
    let video = repository::update_lesson_video(db, lesson_id, dto).await?;
    Ok(video.into())
}


pub async fn delete_lesson_video(
    db: &PgPool,
    lesson_id: i32
) -> Result<(), AppError> {
    let rows_affected = repository::delete_lesson_video_by_lesson_id(db, lesson_id).await?;
    if rows_affected == 0 {
        return Err(AppError::NotFound(format!("Video for lesson_id={} not found", lesson_id)));
    }
    Ok(())
} 