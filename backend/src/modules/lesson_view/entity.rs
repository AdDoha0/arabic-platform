use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LessonTopic {
    pub id: i32,
    pub lesson_id: i32,
    pub topic: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct LessonTheory {
    pub id: i32,
    pub lesson_id: i32,
    pub content: String,
}

#[derive(Debug, Clone, FromRow)]
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


