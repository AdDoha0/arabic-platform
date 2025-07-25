use sqlx::FromRow;

use super::dto::{
    input::CreateLessonVideoDto,
    output::LessonVideoResponseDto,
};


#[derive(Debug, FromRow)]
pub struct LessonVideo {
    pub id: i32,
    pub lesson_id: i32,
    pub title: Option<String>,
    pub youtube_url: String,
}

#[derive(Debug)]
pub struct NewLessonVideo {
    pub lesson_id: i32,
    pub title: Option<String>,
    pub youtube_url: String,
}

// Преобразование DTO → NewLessonVideo

impl From<CreateLessonVideoDto> for NewLessonVideo {
    fn from(dto: CreateLessonVideoDto) -> Self {
        Self {
            lesson_id: dto.lesson_id.expect("lesson_id должен быть передан через path"),
            title: dto.title,
            youtube_url: dto.youtube_url,
        }
    }
}

// Преобразование модели → DTO ответа
impl From<LessonVideo> for LessonVideoResponseDto {
    fn from(video: LessonVideo) -> Self {
        Self {
            id: video.id,
            lesson_id: video.lesson_id,
            title: video.title,
            youtube_url: video.youtube_url,
        }
    }
} 