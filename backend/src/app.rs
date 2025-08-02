use axum::Router;
use crate::app_state::AppState;
use crate::modules::{
    textbooks,
    lessons,
    lesson_video
};

pub fn app_router(state: AppState) -> Router {
    let api_version = "/api/v1";
    
    Router::new()
        .nest(
            &format!("{api_version}/"), 
            textbooks::routes::routes()
        )
        .nest(
            &format!("{api_version}/"), 
            lessons::routes::routes()
        )
        .nest(
            &format!("{api_version}/"), 
            lesson_video::routes::routes()
        )
        .with_state(state)
}