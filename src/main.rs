mod handler;
mod model;
mod route;
mod error;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use std::fs;
use sqlx::sqlite::SqlitePool;
use route::create_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main(){
    const DB_URL: &str = "sqlite://todo.db";
    let env: String = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();
    assert_eq!(key, "DATABASE_URL");
    assert_eq!(database_url,DB_URL);

    let pool = SqlitePool::connect(database_url).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(pool).layer(cors);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
