// src/modules/lesson_view/handler.rs

use axum::{
    extract::{Path, State},
    Json, response::IntoResponse,
};
use crate::common::error::AppError;
use crate::common::response::ApiResponse;
use crate::AppState;
use crate::modules::lesson_view::dto::input::{LessonFullCreateDto, LessonFullUpdateDto};
use crate::modules::lesson_view::dto::output::LessonFullOutputDto;
use crate::modules::lesson_view::service;

/// GET /lessons/:id/full — получить полный урок
pub async fn get_full_lesson_handler(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let lesson: LessonFullOutputDto = service::get_full_lesson(id, &state.dp_pool).await?;
    Ok(ApiResponse::success(lesson))
}

/// POST /lessons/full — создать полный урок
pub async fn create_full_lesson_handler(
    State(state): State<AppState>,
    Json(dto): Json<LessonFullCreateDto>,
) -> Result<impl IntoResponse, AppError> {
    let lesson = service::create_full_lesson(dto, &state.dp_pool).await?;
    Ok(ApiResponse::success(lesson))
}

/// PUT /lessons/:id/full — обновить полный урок
pub async fn update_full_lesson_handler(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(dto): Json<LessonFullUpdateDto>,
) -> Result<impl IntoResponse, AppError> {
    let lesson = service::update_full_lesson(id, dto, &state.dp_pool).await?;
    Ok(ApiResponse::success(lesson))
}

/// DELETE /lessons/:id — удалить полный урок
pub async fn delete_full_lesson_handler(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_full_lesson(id, &state.dp_pool).await?;
    Ok(ApiResponse::success(()))
}
