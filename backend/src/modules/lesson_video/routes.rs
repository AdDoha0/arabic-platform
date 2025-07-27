use axum::{Router, routing::{get, post, patch, delete}};
use crate::AppState;
use super::handlers::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/lessons/{lesson_id}/video",
            get(get_lesson_video_handler)
            .post(create_lesson_video_handler)
            .patch(update_lesson_video_handler)
            .delete(delete_lesson_video_handler)
        )
}