use serde::Serialize;


#[derive(Debug, Serialize)]

pub struct LessonResponseDto {
    pub id: i32,
    pub textbook_id: i32,
    pub title: String,
    pub description: Option<String>,
}
