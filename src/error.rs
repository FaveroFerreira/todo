#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    pub error_message: String,
}

impl ApiError {
    pub fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}
