use sqlx::PgPool;
use crate::common::error::AppError;
use crate::modules::lessons::entity::Lesson as LessonEntity;
use crate::modules::lesson_video::entity::{LessonVideo, NewLessonVideo};
use crate::modules::lesson_view::entity::{
    LessonTopic, LessonTheory, LessonHomework,
    NewLessonTopic, NewLessonTheory, NewLessonHomework,
};
use crate::modules::lesson_view::dto::output::LessonFullOutputDto;

// Вставка тем урока
pub async fn insert_topics(
    db: &PgPool,
    topics: Vec<NewLessonTopic>,
) -> Result<(), AppError> {
    if topics.is_empty() {
        return Ok(());
    }

    let mut query = sqlx::QueryBuilder::new(
        "INSERT INTO lesson_topics (lesson_id, topic) "
    );

    query.push_values(topics, |mut b, topic| {
        b.push_bind(topic.lesson_id)
         .push_bind(topic.topic);
    });

    query
        .build()
        .execute(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

// Вставка теории урока
pub async fn insert_theory(
    db: &PgPool,
    theory: NewLessonTheory,
) -> Result<LessonTheory, AppError> {
    let result = sqlx::query_as!(
        LessonTheory,
        r#"
        INSERT INTO lesson_theory (lesson_id, content)
        VALUES ($1, $2)
        RETURNING id, lesson_id, content
        "#,
        theory.lesson_id,
        theory.content
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}

// Вставка домашнего задания
pub async fn insert_homework(
    db: &PgPool,
    homework: NewLessonHomework,
) -> Result<LessonHomework, AppError> {
    let result = sqlx::query_as!(
        LessonHomework,
        r#"
        INSERT INTO lesson_homework (lesson_id, task)
        VALUES ($1, $2)
        RETURNING id, lesson_id, task
        "#,
        homework.lesson_id,
        homework.task
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(result)
}

// Получение полного урока по ID
pub async fn get_full_lesson_by_id(
    db: &PgPool,
    lesson_id: i32,
) -> Result<LessonFullOutputDto, AppError> {
    // Получаем основной урок
    let lesson = sqlx::query_as!(
        LessonEntity,
        r#"
        SELECT id, textbook_id, title, description, created_at
        FROM lessons
        WHERE id = $1
        "#,
        lesson_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or_else(|| AppError::NotFound(format!("Lesson with id={} not found", lesson_id)))?;

    // Получаем темы урока
    let topics = sqlx::query_as!(
        LessonTopic,
        r#"
        SELECT id, lesson_id, topic
        FROM lesson_topics
        WHERE lesson_id = $1
        ORDER BY id
        "#,
        lesson_id
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Получаем теорию урока
    let theory = sqlx::query_as!(
        LessonTheory,
        r#"
        SELECT id, lesson_id, content
        FROM lesson_theory
        WHERE lesson_id = $1
        "#,
        lesson_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Получаем домашнее задание
    let homework = sqlx::query_as!(
        LessonHomework,
        r#"
        SELECT id, lesson_id, task
        FROM lesson_homework
        WHERE lesson_id = $1
        "#,
        lesson_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Получаем видео урока (используем функцию из lesson_video модуля)
    let video = crate::modules::lesson_video::repository::select_lesson_video_by_lesson_id(db, lesson_id).await?;

    // Собираем результат
    let result = LessonFullOutputDto {
        id: lesson.id,
        textbook_id: lesson.textbook_id,
        title: lesson.title,
        description: lesson.description,
        created_at: lesson.created_at,
        topics: topics.into_iter().map(Into::into).collect(),
        theory: theory.map(Into::into),
        homework: homework.map(Into::into),
        video: video.map(Into::into),
    };

    Ok(result)
}
