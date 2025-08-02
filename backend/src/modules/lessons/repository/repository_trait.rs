use async_trait::async_trait;
use crate::common::error::AppError;
use crate::modules::lesson_video::entity::{LessonVideo, NewLessonVideo};
use crate::modules::lesson_video::dto::input::UpdateLessonVideoDto;


#[async_trait]
pub trait LessonVideoRepository: Send + Sync {
    async fn insert(&self, dto: NewLessonVideo) -> Result<LessonVideo, AppError>;
    async fn update(&self, lesson_id: i32, dto: UpdateLessonVideoDto) -> Result<LessonVideo, AppError>;
    async fn find_by_lesson_id(&self, lesson_id: i32) -> Result<Option<LessonVideo>, AppError>;
    async fn delete_by_lesson_id(&self, lesson_id: i32) -> Result<u64, AppError>;
}