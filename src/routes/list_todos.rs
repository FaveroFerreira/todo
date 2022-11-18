use axum::{Extension, Json};
use sqlx::PgPool;

use crate::entity::todo::Todo;

pub const ENDPOINT: &str = "/todo";

pub async fn handler(Extension(db_pool): Extension<PgPool>) -> Json<Vec<Todo>> {
    todo!("escreva o código aqui")
}
