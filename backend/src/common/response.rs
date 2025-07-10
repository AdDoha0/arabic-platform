use axum::{response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data,
        }
    }
}

impl ApiResponse<serde_json::Value> {
    pub fn message(msg: &str) -> Self {
        Self {
            status: "success".to_string(),
            data: serde_json::json!({ "message": msg }),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
