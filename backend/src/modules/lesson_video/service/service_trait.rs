use async_trait::async_trait;
use crate::common::error::AppError;
use crate::modules::lesson_video::dto::input::{CreateLessonVideoDto, UpdateLessonVideoDto};
use crate::modules::lesson_video::dto::output::LessonVideoResponseDto;
use crate::modules::lesson_video::repository::repository_trait::LessonVideoRepository;


#[async_trait]
pub trait LessonVideoService: Send + Sync {
    async fn get_lesson_video(&self, lesson_id: i32) -> Result<LessonVideoResponseDto, AppError>;
    async fn create_lesson_video(&self, dto: CreateLessonVideoDto) -> Result<LessonVideoResponseDto, AppError>;
    async fn update_lesson_video(&self, lesson_id: i32, dto: UpdateLessonVideoDto) -> Result<LessonVideoResponseDto, AppError>;
    async fn delete_lesson_video(&self, lesson_id: i32) -> Result<(), AppError>;
}




