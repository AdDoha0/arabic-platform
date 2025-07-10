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
    pub status: String,
    pub message: String,
    pub r#type: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, message, error_type) = match self {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg, "database"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, "not_found"),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg, "validation"),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg, "internal"),
        };

        let body = Json(ErrorResponse {
            status: "error".to_string(),
            message,
            r#type: error_type.to_string()
        });

        (status_code, body).into_response()
    }
}