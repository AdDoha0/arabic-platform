use async_trait::async_trait;
use crate::modules::lessons::{    
    dto::{input::{CreateLessonDto, UpdateLessonDto}, output::LessonResponseDto},
    entity::NewLesson,
    query::LessonQuery,
    repository::{self, repository_trait::LessonRepository}
};

use super::service_trait::LessonService;

use crate::common::{
    error::AppError, query_params::pagination::HasPagination, response::PaginatedResponse
};


#[derive(Clone)]
pub struct LessonServiceImpl<R>
where
    R: LessonRepository,
{
    repository: R,
}


impl<R> LessonServiceImpl<R>
where
    R: LessonRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> LessonService for LessonServiceImpl<R>
where
    R: LessonRepository + Send + Sync,
{ 

    async fn get_lessons(&self, params: &LessonQuery) -> Result<PaginatedResponse<LessonResponseDto>, AppError> {
        let total = self.repository.count_lessons().await?;
        let lessons = self.repository.select_all_lessons(params).await?;
        
        let dto = lessons.into_iter().map(Into::into).collect();
        
        Ok(PaginatedResponse::new(
            dto, 
            total, 
            params.page_or_default(), 
            params.limit_or_default()
        ))
    }

    async fn get_lesson_by_id(&self, id: i32) -> Result<LessonResponseDto, AppError> {
        let lesson = self.repository.select_lesson_by_id(id).await?;
        let lesson = lesson.ok_or(AppError::NotFound(format!("Lesson with id={} not found", id)))?;
        Ok(lesson.into())
    }

    async fn create_lesson(&self, dto: CreateLessonDto) -> Result<LessonResponseDto, AppError> {
        let new_lesson = NewLesson::from(dto);
        let lesson = self.repository.insert_lesson(new_lesson).await?;
        Ok(lesson.into())
    }

    async fn update_lesson(&self, id: i32, dto: UpdateLessonDto) -> Result<LessonResponseDto, AppError> {
        let lesson = self.repository.update_lesson_by_id(id, dto).await?;
        Ok(lesson.into())
    }

    async fn delete_lesson(&self, id: i32) -> Result<(), AppError> {
        let rows_affected = self.repository.delete_lesson_by_id(id).await?;
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Lesson with id={} not found", id)));
        }
        Ok(())
    }

}

