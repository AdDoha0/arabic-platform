use std::env;
use sqlx::PgPool;
use tokio::net::TcpListener;
use app::app_router;
use db::AppState;

mod modules;
mod app;
mod db;
mod common;

#[tokio::main]
async fn main() {

    println!("http://localhost:2000/");

    dotenv::dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("Connect database url!");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let app = app_router(AppState { dp_pool: db_pool});

    let listener = TcpListener::bind("0.0.0.0:2000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
