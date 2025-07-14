use serde::Deserialize;


#[derive(Debug, Deserialize)]

pub struct CreateLessonDto {
    pub textbook_id: i32,
    pub title: String,
    pub description: Option<String>,
}  


#[derive(Debug, Deserialize)]

pub struct UpdateLessonDto {
    pub textbook_id: i32,
    pub title: String,
    pub description: Option<String>,
}