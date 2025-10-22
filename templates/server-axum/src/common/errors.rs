use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Something went wrong with  the database operation {:?}",
                    error
                ),
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };

        let body = Json(serde_json::json!({
        "success": false, 
        "error": error_message}));

        (status, body).into_response()
    }
}
