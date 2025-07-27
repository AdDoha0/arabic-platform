use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;

// use super::dto::input::{CreateLessonTopicDto, CreateLessonTheoryDto, CreateLessonHomeworkDto};
// use super::dto::output::{LessonTopicDto, LessonTheoryDto, LessonHomeworkDto};


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LessonTopic {
    pub id: i32,
    pub lesson_id: i32,
    pub topic: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LessonTheory {
    pub id: i32,
    pub lesson_id: i32,
    pub content: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LessonHomework {
    pub id: i32,
    pub lesson_id: i32,
    pub task: String,
}


#[derive(Debug)]
pub struct NewLessonTopic {
    pub lesson_id: i32,
    pub topic: String,
}


#[derive(Debug)]
pub struct NewLessonTheory {
    pub lesson_id: i32,
    pub content: String,
}


#[derive(Debug)]
pub struct NewLessonHomework {
    pub lesson_id: i32,
    pub task: String,
}


