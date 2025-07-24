use sqlx::FromRow;

use super::dto::{
    input::CreateLessonDto,
    output::LessonResponseDto, 
};


#[derive(Debug, FromRow)]
pub struct Lesson {
    pub id: i32,
    pub textbook_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String
}


#[derive(Debug, FromRow)]
pub struct NewLesson {
    pub textbook_id: i32, 
    pub title: String,
    pub description: Option<String>,
}

// Преобразование DTO → NewLesson
impl From<CreateLessonDto> for NewLesson{
    fn from(dto: CreateLessonDto) -> Self {
        Self {
            textbook_id: dto.textbook_id, 
            title: dto.title,
            description: dto.description, 
        }
    }
}

// Преобразование модели → DTO ответа
impl From<Lesson> for LessonResponseDto  {
    fn from(tb: Lesson) -> Self {
        Self {
            id: tb.id,
            textbook_id: tb.textbook_id,
            title: tb.title, 
            description: tb.description,
            created_at: tb.created_at
        }
    }    
}
