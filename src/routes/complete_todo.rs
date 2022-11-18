use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Extension, Json,
};
use hyper::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{entity::todo::Todo, error::ApiError};

pub const ENDPOINT: &str = "/todo/:id";

pub async fn handler(
    Extension(db_pool): Extension<PgPool>,
    Path(todo_id): Path<Uuid>,
) -> Result<(), Response> {
    let todo = sqlx::query_as::<_, Todo>(r#"SELECT * FROM todo WHERE id = $1"#)
        .bind(todo_id)
        .fetch_optional(&db_pool)
        .await
        .unwrap();

    if todo.is_none() {
        let response_body = ApiError::new("todo not found!");
        let response = (StatusCode::NOT_FOUND, Json(response_body));
        return Err(response.into_response());
    }

    sqlx::query("UPDATE todo SET is_completed = true WHERE id = $1")
        .bind(todo_id)
        .execute(&db_pool)
        .await
        .unwrap();

    Ok(())
}
