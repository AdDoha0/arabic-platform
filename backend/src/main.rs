use std::env;
use sqlx::PgPool;
use tokio::net::TcpListener;
use app::app_router;
use app_state::AppState;

mod modules;
mod app;
mod app_state;
mod common;

#[tokio::main]
async fn main() {

    println!("http://localhost:2000/");

    dotenv::dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("Connect database url!");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let app_state = AppState::new(db_pool);
    let app = app_router(app_state);

    let listener = TcpListener::bind("0.0.0.0:2000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
