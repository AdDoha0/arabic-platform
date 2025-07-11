use axum::Router;
use crate::AppState;
use crate::modules::textbooks;

pub fn app_router(state: AppState) -> Router {
    let api_version = "/api/v1";
    
    Router::new()
        .nest(
            &format!("{api_version}/"), 
            textbooks::routes::routes()
        )
        .with_state(state)
}