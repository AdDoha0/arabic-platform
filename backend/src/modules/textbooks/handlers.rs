use std::{ops::ControlFlow, os::linux::raw::stat};

use axum::{extract::{State, Path}, response::IntoResponse, Json};
use crate::{
    AppState,
    common::error::AppError,
    common::response::ApiResponse,
    modules::textbooks::{
        dto::input::{CreateTextbookDto, UpdateTextbookDto},
        service::{
            create_textbook,
            delete_textbook,
            get_textbook_by_id,
            list_textbooks,
            patch_textbook,
        },
    },
};

// НАДО ДОБАВИТЬ:
// limit / page или offset / limit;
// фильтрация по полям (level, is_active);
// сортировка (sort_by=title, order=desc).
// envelope 


pub async fn create_textbook_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateTextbookDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = create_textbook(&state.dp_pool, payload).await?;
    Ok(ApiResponse::success(result))
}


pub async fn get_textbook_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let result = get_textbook_by_id(&state.dp_pool, id).await?;
    Ok(ApiResponse::success(result))
}

pub async fn list_textbooks_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result: Vec<super::dto::output::TextbookResponseDto> = list_textbooks(&state.dp_pool).await?;
    Ok(ApiResponse::success(result))
}

pub async fn update_textbook_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateTextbookDto>,
) -> Result<impl IntoResponse, AppError> {
    let result = patch_textbook(&state.dp_pool, id, dto).await?;
    Ok(ApiResponse::success(result))
}

pub async fn delete_textbook_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, AppError> { 
    delete_textbook(&state.dp_pool, id).await?; 
    Ok(ApiResponse::message("deleted"))
}