use async_trait::async_trait;
use crate::modules::lessons::{    
    dto::input::{CreateLessonDto, UpdateLessonDto},
    dto::output::LessonResponseDto,
    entity::NewLesson,
    query::LessonQuery,
    repository::repository_trait::LessonRepository
};

use crate::common::{
    error::AppError,
    query_params::pagination::HasPagination,
    response::PaginatedResponse
};

#[async_trait]
pub trait LessonService: Send + Sync {
    async fn get_lessons(&self, params: &LessonQuery) -> Result<PaginatedResponse<LessonResponseDto>, AppError>;
    async fn get_lesson_by_id(&self, id: i32) -> Result<LessonResponseDto, AppError>;
    async fn create_lesson(&self, dto: CreateLessonDto) -> Result<LessonResponseDto, AppError>;
    async fn update_lesson(&self, id: i32, dto: UpdateLessonDto) -> Result<LessonResponseDto, AppError>;
    async fn delete_lesson(&self, id: i32) -> Result<(), AppError>;
}