use axum::Router;
use crate::AppState;
use crate::modules::{
    textbooks,
    lessons
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
        .with_state(state)
}