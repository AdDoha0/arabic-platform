use axum::extract::Query;
use axum::{extract::{State, Path}, response::IntoResponse, Json};
use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
};

use super::{
    dto::input::{CreateLessonDto, UpdateLessonDto},
    query::LessonQuery,

};



pub async fn create_lesson_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateLessonDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson().create_lesson(payload).await?;
    Ok(ApiResponse::success(result))
}


pub async fn get_lesson_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson().get_lesson_by_id(id).await?;
    Ok(ApiResponse::success(result))
}


pub async fn list_lessons_handler(
    State(state): State<AppState>,
    Query(pagination): Query<LessonQuery>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson().get_lessons(&pagination).await?;
    Ok(ApiResponse::success(result))
}



pub async fn update_lesson_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateLessonDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.services().lesson().update_lesson(id, dto).await?;
    Ok(ApiResponse::success(result))
}


pub async fn delete_lesson_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, AppError> { 
    state.services().lesson().delete_lesson(id).await?; 
    Ok(ApiResponse::message("deleted"))
}