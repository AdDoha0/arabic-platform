use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize; 

use std::io;
use std::num::{ParseIntError, TryFromIntError};
use std::str::Utf8Error;
use serde_json;
use sqlx;
use std::string::FromUtf8Error;
use std::time::SystemTimeError;
use chrono::ParseError as ChronoParseError;


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

// SQLX
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

// IO
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

// JSON
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

// String → UTF-8
impl From<FromUtf8Error> for AppError {
    fn from(err: FromUtf8Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

// System time errors
impl From<SystemTimeError> for AppError {
    fn from(err: SystemTimeError) -> Self {
        AppError::Internal(err.to_string())
    }
}

// Парсинг чисел
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<TryFromIntError> for AppError {
    fn from(err: TryFromIntError) -> Self {
        AppError::Internal(err.to_string())
    }
}

// Chrono парсинг
impl From<ChronoParseError> for AppError {
    fn from(err: ChronoParseError) -> Self {
        AppError::Internal(err.to_string())
    }
}
