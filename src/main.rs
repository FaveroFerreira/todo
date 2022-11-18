use axum::routing::{get, patch, post};
use axum::{Extension, Router};
use sqlx::PgPool;

mod dto;
mod entity;
mod error;
mod routes;

#[tokio::main]
async fn main() {
    let db_pool = PgPool::connect("postgres://root:root@localhost:5432/todo_db")
        .await
        .unwrap();

    let app = Router::new()
        .route(
            routes::create_todo::ENDPOINT,
            post(routes::create_todo::handler),
        )
        .route(
            routes::list_todos::ENDPOINT,
            get(routes::list_todos::handler),
        )
        .route(
            routes::complete_todo::ENDPOINT,
            patch(routes::complete_todo::handler),
        )
        .layer(Extension(db_pool));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
