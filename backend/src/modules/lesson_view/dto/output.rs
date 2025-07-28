use serde::Serialize;

use crate::modules::lesson_view::entity::{
    LessonTopic, LessonTheory, LessonHomework,
};
use crate::modules::lesson_video::entity::LessonVideo;

#[derive(Debug, Serialize)]
pub struct LessonFullOutputDto {
    pub id: i32,
    pub textbook_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,

    pub topics: Vec<LessonTopicDto>,
    pub theory: Option<LessonTheoryDto>,
    pub homework: Option<LessonHomeworkDto>,
    pub video: Option<LessonVideoDto>,
}

#[derive(Debug, Serialize)]
pub struct LessonVideoDto {
    pub id: i32, 
    pub title: Option<String>,
    pub youtube_url: String,
}


#[derive(Debug, Serialize)]
pub struct LessonTopicDto {
    pub id: i32,
    pub topic: String,
}

#[derive(Debug, Serialize)]
pub struct LessonTheoryDto {
    pub id: i32,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct LessonHomeworkDto {
    pub id: i32,
    pub task: String,
}


// Конвертации: Entity → Output DTO
impl From<LessonVideo> for LessonVideoDto {
    fn from(entity: LessonVideo) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            youtube_url: entity.youtube_url
        }
    }
}


impl From<LessonTopic> for LessonTopicDto {
    fn from(entity: LessonTopic) -> Self {
        Self {
            id: entity.id,
            topic: entity.topic,
        }
    }
}

impl From<LessonTheory> for LessonTheoryDto {
    fn from(entity: LessonTheory) -> Self {
        Self {
            id: entity.id,
            content: entity.content,
        }
    }
}

impl From<LessonHomework> for LessonHomeworkDto {
    fn from(entity: LessonHomework) -> Self {
        Self {
            id: entity.id,
            task: entity.task,
        }
    }
}
