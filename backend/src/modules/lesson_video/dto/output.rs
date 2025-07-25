use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LessonVideoResponseDto {
    pub id: i32,
    pub lesson_id: i32,
    pub title: Option<String>,
    pub youtube_url: String,
} 