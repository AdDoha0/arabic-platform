// src/modules/lesson_view/repository.rs

use sqlx::{PgPool, Postgres, Transaction};
use crate::common::error::AppError;
use crate::modules::lessons::entity::{Lesson, NewLesson};
use crate::modules::lesson_video::entity::{LessonVideo, NewLessonVideo};
use crate::modules::lesson_view::entity::{
    LessonTopic, NewLessonTopic,
    LessonTheory, NewLessonTheory,
    LessonHomework, NewLessonHomework,
};
use crate::modules::lesson_view::dto::output::LessonFullOutputDto;

// --- CRUD for base lesson ---
pub async fn insert_lesson_tx(
    tx: &mut Transaction<'_, Postgres>,
    dto: NewLesson
) -> Result<Lesson, AppError> {
    let lesson = sqlx::query_as!(
        Lesson,
        r#"INSERT INTO lessons (textbook_id, title, description)
        VALUES ($1, $2, $3)
        RETURNING id, textbook_id, title, description, created_at"#,
        dto.textbook_id, dto.title, dto.description
    )
    .fetch_one(&mut *tx)
    .await?;

    Ok(lesson)
}

pub async fn update_lesson_by_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    id: i32,
    dto: crate::modules::lesson_view::dto::input::LessonFullUpdateDto,
) -> Result<Lesson, AppError> {
    // Only update provided fields
    let lesson = sqlx::query_as!(
        Lesson,
        r#"UPDATE lessons SET
           textbook_id = COALESCE($2, textbook_id),
           title       = COALESCE($3, title),
           description = COALESCE($4, description)
           WHERE id = $1
           RETURNING id, textbook_id, title, description, created_at"#,
        id, dto.textbook_id, dto.title, dto.description
    )
    .fetch_one(&mut *tx)
    .await?;

    Ok(lesson)
}

pub async fn delete_full_lesson_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
) -> Result<(), AppError> {
    // Delete children
    sqlx::query!("DELETE FROM lesson_topics WHERE lesson_id = $1", lesson_id)
        .execute(&mut *tx).await?;
    sqlx::query!("DELETE FROM lesson_theory WHERE lesson_id = $1", lesson_id)
        .execute(&mut *tx).await?;
    sqlx::query!("DELETE FROM lesson_homework WHERE lesson_id = $1", lesson_id)
        .execute(&mut *tx).await?;
    sqlx::query!("DELETE FROM lessons_videos WHERE lesson_id = $1", lesson_id)
        .execute(&mut *tx).await?;
    // Then delete lesson
    sqlx::query!("DELETE FROM lessons WHERE id = $1", lesson_id)
        .execute(&mut *tx).await?;

    Ok(())
}

// --- CRUD for topics ---

pub async fn insert_topics_tx(
    tx: &mut Transaction<'_, Postgres>,
    topics: Vec<NewLessonTopic>,
) -> Result<(), AppError> {
    if topics.is_empty() { return Ok(()); }
    let mut builder = sqlx::QueryBuilder::new(
        "INSERT INTO lesson_topics (lesson_id, topic)"
    );
    builder.push_values(topics, |mut b, t| {
        b.push_bind(t.lesson_id).push_bind(t.topic.clone());
    });
    builder.build().execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn delete_topics_by_lesson(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32
) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM lesson_topics WHERE lesson_id = $1", lesson_id)
        .execute(&mut *tx)?;
    Ok(())
}

// --- CRUD for theory ---

pub async fn insert_theory_tx(
    tx: &mut Transaction<'_, Postgres>,
    dto: NewLessonTheory
) -> Result<LessonTheory, AppError> {
    let res = sqlx::query_as!(
        LessonTheory,
        "INSERT INTO lesson_theory (lesson_id, content) VALUES ($1,$2) RETURNING id,lesson_id,content",
        dto.lesson_id, dto.content
    )
    .fetch_one(&mut *tx).await?;
    Ok(res)
}

pub async fn delete_theory_by_lesson(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32
) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM lesson_theory WHERE lesson_id=$1", lesson_id)
        .execute(&mut *tx).await?;
    Ok(())
}

// --- CRUD for homework ---

pub async fn insert_homework_tx(
    tx: &mut Transaction<'_, Postgres>,
    dto: NewLessonHomework
) -> Result<LessonHomework, AppError> {
    let res = sqlx::query_as!(
        LessonHomework,
        "INSERT INTO lesson_homework (lesson_id,task) VALUES ($1,$2) RETURNING id,lesson_id,task",
        dto.lesson_id, dto.task
    )
    .fetch_one(&mut *tx).await?;
    Ok(res)
}

pub async fn delete_homework_by_lesson(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32
) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM lesson_homework WHERE lesson_id=$1", lesson_id)
        .execute(&mut *tx).await?;
    Ok(())
}

// --- CRUD for video ---

pub async fn insert_lesson_video_tx(
    tx: &mut Transaction<'_, Postgres>,
    dto: NewLessonVideo
) -> Result<LessonVideo, AppError> {
    let res = sqlx::query_as!(
        LessonVideo,
        "INSERT INTO lessons_videos (lesson_id,title,youtube_url) VALUES ($1,$2,$3) RETURNING id,lesson_id,title,youtube_url",
        dto.lesson_id, dto.title, dto.youtube_url
    )
    .fetch_one(&mut *tx).await?;
    Ok(res)
}

pub async fn delete_video_by_lesson(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32
) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM lessons_videos WHERE lesson_id=$1", lesson_id)
        .execute(&mut *tx).await?;
    Ok(())
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
    .await?;

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
    .await?;

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
    .await?;

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
    .await?;

    // Получаем видео урока
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

