use serde::Deserialize;

use crate::modules::lesson_view::entity::{
    NewLessonTopic, NewLessonTheory, NewLessonHomework,
};

use crate::modules::lesson_video::entity::NewLessonVideo;


pub trait IntoNewWithLessonId {
    type Output;

    fn into_new(self, lesson_id: i32) -> Self::Output;
}


#[derive(Debug, Deserialize)]
pub struct LessonFullCreateDto {
    pub textbook_id: i32,
    pub title: String,
    pub description: Option<String>,

    pub topics: Vec<CreateLessonTopicDto>,
    pub theory: Option<CreateLessonTheoryDto>,
    pub homework: Option<CreateLessonHomeworkDto>,
    pub video: Option<CreateLessonVideoDto>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LessonFullUpdateDto {
    pub textbook_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,

    pub topics: Option<Vec<CreateLessonTopicDto>>,
    pub theory: Option<CreateLessonTheoryDto>,
    pub homework: Option<CreateLessonHomeworkDto>,
    pub video: Option<CreateLessonVideoDto>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLessonVideoDto {
    pub title: Option<String>,
    pub youtube_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLessonTopicDto {
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLessonTheoryDto {
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateLessonHomeworkDto {
    pub task: String,
}

impl IntoNewWithLessonId for CreateLessonVideoDto {
    type Output = NewLessonVideo;

    fn into_new(self, lesson_id: i32) -> Self::Output {
        NewLessonVideo {
            lesson_id,
            title: self.title,
            youtube_url: self.youtube_url,
        }
    }
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