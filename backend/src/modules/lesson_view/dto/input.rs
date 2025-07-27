use serde::Deserialize;

use crate::modules::lesson_view::entity::{
    NewLessonTopic, NewLessonTheory, NewLessonHomework,
};


#[derive(Debug, Deserialize)]
pub struct CreateLessonTopicDto {
    pub topic: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLessonTheoryDto {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLessonHomeworkDto {
    pub task: String,
}

pub trait IntoNewWithLessonId {
    type Output;

    fn into_new(self, lesson_id: i32) -> Self::Output;
}

impl IntoNewWithLessonId for CreateLessonTopicDto {
    type Output = NewLessonTopic;

    fn into_new(self, lesson_id: i32) -> Self::Output {
        NewLessonTopic {
            lesson_id,
            topic: self.topic,
        }
    }
}

impl IntoNewWithLessonId for CreateLessonTheoryDto {
    type Output = NewLessonTheory;

    fn into_new(self, lesson_id: i32) -> Self::Output {
        NewLessonTheory {
            lesson_id,
            content: self.content,
        }
    }
}

impl IntoNewWithLessonId for CreateLessonHomeworkDto {
    type Output = NewLessonHomework;

    fn into_new(self, lesson_id: i32) -> Self::Output {
        NewLessonHomework {
            lesson_id,
            task: self.task,
        }
    }
}