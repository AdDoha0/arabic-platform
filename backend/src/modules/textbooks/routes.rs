use axum::{Router, routing::{get, post, patch, delete}};
use crate::AppState;

use super::handlers::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/textbooks", 
        get(list_textbooks_handler)
        .post(create_textbook_handler))
        .route("/textbooks/{id}",
            get(get_textbook_handler)
            .delete(delete_textbook_handler)
            .patch(update_textbook_handler)
        )
}
