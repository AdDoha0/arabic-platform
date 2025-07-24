use axum::{Router, routing::{get, post, patch, delete}};
use crate::AppState;

use super::handlers::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/textbooks", 
        get(list_lessons_handler)
        .post(create_lesson_handler))
        .route("/textbooks/{id}",
            get(get_lesson_handler)
            .delete(delete_lesson_handler)
            .patch(update_lesson_handler)
        )
}
