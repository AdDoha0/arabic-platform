use async_trait::async_trait;
use crate::common::error::AppError;
use crate::modules::lessons::entity::{Lesson, NewLesson};
use crate::modules::lessons::dto::input::UpdateLessonDto;
use crate::modules::lessons::query::LessonQuery;


#[async_trait]
pub trait LessonRepository: Send + Sync {
    async fn select_all_lessons(&self, params: &LessonQuery) -> Result<Vec<Lesson>, AppError>;
    async fn count_lessons(&self) -> Result<i64, AppError>;
    async fn insert_lesson(&self, dto: NewLesson) -> Result<Lesson, AppError>;
    async fn select_lesson_by_id(&self, id: i32) -> Result<Option<Lesson>, AppError>;
    async fn delete_lesson_by_id(&self, id: i32) -> Result<u64, AppError>;
    async fn update_lesson_by_id(&self, id: i32, dto: UpdateLessonDto) -> Result<Lesson, AppError>;
}