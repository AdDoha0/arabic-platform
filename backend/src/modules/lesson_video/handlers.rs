use axum::{extract::{State, Path}, response::IntoResponse, Json};
use crate::{app_state::AppState, common::error::AppError, common::response::ApiResponse};
use super::dto::input::{CreateLessonVideoDto, UpdateLessonVideoDto};

pub async fn get_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson_video().get_lesson_video(lesson_id).await?;
    Ok(ApiResponse::success(result))
}

pub async fn create_lesson_video_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateLessonVideoDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson_video().create_lesson_video(payload).await?;
    Ok(ApiResponse::success(result))
}

pub async fn update_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
    Json(payload): Json<UpdateLessonVideoDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson_video().update_lesson_video(lesson_id, payload).await?;
    Ok(ApiResponse::success(result))
}

pub async fn delete_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    state.services().lesson_video().delete_lesson_video(lesson_id).await?;
    Ok(ApiResponse::message("deleted"))
}