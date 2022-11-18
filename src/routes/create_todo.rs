use axum::{Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dto::create_todo::CreateTodoDto, entity::todo::Todo};

pub const ENDPOINT: &str = "/todo";

pub async fn handler(
    Extension(db_pool): Extension<PgPool>,
    Json(body): Json<CreateTodoDto>,
) -> Json<Todo> {
    todo!("escreva o código aqui")
}
