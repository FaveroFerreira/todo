use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTodoDto {
    pub description: String,
}
