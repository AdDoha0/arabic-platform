use sqlx::FromRow;

use super::dto::{
    input::CreateTextbookDto,
    output::TextbookResponseDto, 
};

#[derive(Debug, FromRow)]
pub struct Textbook {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>, 
    pub is_active: bool,
}

#[derive(Debug)]
pub struct NewTextbook {
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>,
    pub is_active: bool,
}


// Преобразование DTO → NewTextbook
impl From<CreateTextbookDto> for NewTextbook {
    fn from(dto: CreateTextbookDto) -> Self {
        Self {
            title: dto.title,
            description: dto.description,
            level: dto.level, 
            is_active: true
        }
    }
}


// Преобразование модели → DTO ответа
impl From<Textbook> for TextbookResponseDto {
    fn from(tb: Textbook) -> Self {
        Self {
            id: tb.id,
            title: tb.title,
            description: tb.description,
            level: tb.level,
            is_active: tb.is_active,
        }
    }
}