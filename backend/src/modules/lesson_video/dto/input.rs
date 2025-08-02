use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateLessonVideoDto {
    pub lesson_id: i32,
    pub title: Option<String>,
    pub youtube_url: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLessonVideoDto {
    pub title: Option<String>,
    pub youtube_url: Option<String>,
} 