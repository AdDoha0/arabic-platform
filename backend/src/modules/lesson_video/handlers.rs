use axum::{extract::{State, Path}, response::IntoResponse, Json};
use crate::{AppState, common::error::AppError, common::response::ApiResponse};
use super::dto::input::{CreateLessonVideoDto, UpdateLessonVideoDto};
use super::service::{get_lesson_video, create_lesson_video, update_lesson_video, delete_lesson_video};

pub async fn get_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = get_lesson_video(&state.dp_pool, lesson_id).await?;
    Ok(ApiResponse::success(result))
}


pub async fn create_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
    Json(mut payload): Json<CreateLessonVideoDto>,
) -> Result<impl IntoResponse, AppError> {
    payload.lesson_id = Some(lesson_id);
    let result = create_lesson_video(&state.dp_pool, payload).await?;
    Ok(ApiResponse::success(result))
}


pub async fn update_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
    Json(payload): Json<UpdateLessonVideoDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = update_lesson_video(&state.dp_pool, lesson_id, payload).await?;
    Ok(ApiResponse::success(result))
}


pub async fn delete_lesson_video_handler(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    delete_lesson_video(&state.dp_pool, lesson_id).await?;
    Ok(ApiResponse::message("deleted"))
} 