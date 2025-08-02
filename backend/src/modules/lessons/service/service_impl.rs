use async_trait::async_trait;
use crate::common::error::AppError;
use crate::modules::lesson_video::{
    repository::repository_trait::LessonVideoRepository,
    entity::NewLessonVideo, 
    dto::input::{CreateLessonVideoDto, UpdateLessonVideoDto},
    dto::output::LessonVideoResponseDto,  
};

use super::service_trait::LessonVideoService; 

#[derive(Clone)]
pub struct LessonVideoServiceImpl<R>
where
    R: LessonVideoRepository,
{
    repository: R,
}


impl<R> LessonVideoServiceImpl<R>
where
    R: LessonVideoRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}


#[async_trait]
impl<R> LessonVideoService for LessonVideoServiceImpl<R>
where
    R: LessonVideoRepository + Send + Sync,
{
    async fn get_lesson_video(&self, lesson_id: i32) -> Result<LessonVideoResponseDto, AppError> {
        let video = self.repository.find_by_lesson_id(lesson_id).await?;
        let video = video.ok_or(AppError::NotFound(format!("Video for lesson_id={} not found", lesson_id)))?;
        Ok(video.into())
    }

    async fn create_lesson_video(&self, dto: CreateLessonVideoDto) -> Result<LessonVideoResponseDto, AppError> {
        let new_video = NewLessonVideo::from(dto);
        let video = self.repository.insert(new_video).await?;
        Ok(video.into())
    }

    async fn update_lesson_video(&self, lesson_id: i32, dto: UpdateLessonVideoDto) -> Result<LessonVideoResponseDto, AppError> {
        let video = self.repository.update(lesson_id, dto).await?;
        Ok(video.into())
    }

    async fn delete_lesson_video(&self, lesson_id: i32) -> Result<(), AppError> {
        let rows_affected = self.repository.delete_by_lesson_id(lesson_id).await?;
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Video for lesson_id={} not found", lesson_id)));
        }
        Ok(())
    }
}
