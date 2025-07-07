use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize; 

#[derive(Debug)]
pub enum AppError {
    Database(String),
    NotFound(String),
    Validation(String),
    Internal(String)
}


#[derive(Serialize)]
struct ErrorResponse {
    error: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
    let body = Json(ErrorResponse { error: message });

    (status, body).into_response()

    } 
}