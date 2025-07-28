// src/modules/lesson_view/service.rs

use sqlx::{PgPool, Postgres, Transaction};
use crate::common::error::AppError;
use crate::modules::lessons::entity::NewLesson;
use crate::modules::lesson_video::entity::NewLessonVideo;
use crate::modules::lesson_view::dto::input::{LessonFullCreateDto, LessonFullUpdateDto, IntoNewWithLessonId};
use crate::modules::lesson_view::dto::output::LessonFullOutputDto;
use crate::modules::lesson_view::entity::{NewLessonTopic, NewLessonTheory, NewLessonHomework};
use crate::modules::lesson_view::repository;


pub async fn get_full_lesson(
    lesson_id: i32,
    pool: &PgPool,
) -> Result<LessonFullOutputDto, AppError> {
    repository::get_full_lesson_by_id(pool, lesson_id).await
}


pub async fn create_full_lesson(
    dto: LessonFullCreateDto,
    pool: &PgPool,
) -> Result<LessonFullOutputDto, AppError> {
    // Начинаем транзакцию и мапим ошибку
    let mut tx: Transaction<'_, Postgres> = pool
        .begin()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Вставляем базовый Lesson
    let lesson = repository::insert_lesson_tx(
        &mut tx,
        NewLesson {
            textbook_id: dto.textbook_id,
            title: dto.title.clone(),
            description: dto.description.clone(),
        },
    )
    .await?;

    // Вставляем вложенные части
    let topics: Vec<NewLessonTopic> = dto
        .topics
        .into_iter()
        .map(|d| d.into_new(lesson.id))
        .collect();
    repository::insert_topics_tx(&mut tx, topics).await?;

    if let Some(th) = dto.theory {
        repository::insert_theory_tx(&mut tx, th.into_new(lesson.id)).await?;
    }
    if let Some(hw) = dto.homework {
        repository::insert_homework_tx(&mut tx, hw.into_new(lesson.id)).await?;
    }
    if let Some(v) = dto.video {
        repository::insert_lesson_video_tx(&mut tx, v.into_new(lesson.id)).await?;
    }

    // Фиксируем транзакцию и мапим ошибку
    tx.commit()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Возвращаем результат
    repository::get_full_lesson_by_id(pool, lesson.id).await
}


pub async fn update_full_lesson(
    lesson_id: i32,
    dto: LessonFullUpdateDto,
    pool: &PgPool,
) -> Result<LessonFullOutputDto, AppError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Обновляем базовый урок
    repository::update_lesson_by_id_tx(&mut tx, lesson_id, dto.clone()).await?;

    // Обновляем вложенные части по наличию
    if let Some(topics) = dto.topics {
        repository::delete_topics_by_lesson(&mut tx, lesson_id).await?;
        let ents = topics.into_iter().map(|d| d.into_new(lesson_id)).collect();
        repository::insert_topics_tx(&mut tx, ents).await?;
    }
    if let Some(th) = dto.theory {
        repository::delete_theory_by_lesson(&mut tx, lesson_id).await?;
        repository::insert_theory_tx(&mut tx, th.into_new(lesson_id)).await?;
    }
    if let Some(hw) = dto.homework {
        repository::delete_homework_by_lesson(&mut tx, lesson_id).await?;
        repository::insert_homework_tx(&mut tx, hw.into_new(lesson_id)).await?;
    }
    if let Some(v) = dto.video {
        repository::delete_video_by_lesson(&mut tx, lesson_id).await?;
        repository::insert_lesson_video_tx(&mut tx, v.into_new(lesson_id)).await?;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    repository::get_full_lesson_by_id(pool, lesson_id).await
}


/// DELETE /lessons/:id
pub async fn delete_full_lesson(
    lesson_id: i32,
    pool: &PgPool,
) -> Result<(), AppError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    repository::delete_full_lesson_tx(&mut tx, lesson_id).await?;

    tx.commit()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}
