use serde::Serialize;

use crate::modules::lesson_view::entity::{
    LessonTopic, LessonTheory, LessonHomework,
};

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
