use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database query error")]
    ErrorQueryingDatabase,
    #[error("User already exists")]
    ExistingUser,
    #[error("Password hashing failed")]
    PasswordHashingFailed,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::ErrorQueryingDatabase => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while querying database, please try again",
            ),
            Self::ExistingUser => (StatusCode::CONFLICT, "User with email already exists"),
            Self::PasswordHashingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Passing hashing failed. Try again",
            ),
        };

        let body = Json(serde_json::json!({
        "success": false, 
        "error": error_message}));

        (status, body).into_response()
    }
}
