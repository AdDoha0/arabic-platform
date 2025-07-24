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
    service::{
        create_lesson,
        delete_lesson,
        get_lesson_by_id,
        list_lessons,
        patch_lesson,
    },
};



pub async fn create_lesson_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateLessonDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = create_lesson(&state.dp_pool, payload).await?;
    Ok(ApiResponse::success(result))
}


pub async fn get_lesson_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = get_lesson_by_id(&state.dp_pool, id).await?;
    Ok(ApiResponse::success(result))
}


pub async fn list_lessons_handler(
    State(state): State<AppState>,
    Query(pagination): Query<LessonQuery>,
) -> Result<impl IntoResponse, AppError> {
    let result = list_lessons(&state.dp_pool, pagination).await?;
    Ok(ApiResponse::success(result))
}



pub async fn update_lesson_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateLessonDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = patch_lesson(&state.dp_pool, id, dto).await?;
    Ok(ApiResponse::success(result))
}


pub async fn delete_lesson_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, AppError> { 
    delete_lesson(&state.dp_pool, id).await?; 
    Ok(ApiResponse::message("deleted"))
}