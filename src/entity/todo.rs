use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub description: String,
    pub is_completed: bool,
}
