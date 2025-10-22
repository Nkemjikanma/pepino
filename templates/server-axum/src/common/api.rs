use axum::Json;
use serde::Serialize;

use super::errors::AppError;

#[derive(Debug, Serialize)]
pub struct APIResponse<T> {
    pub response_message: String,
    pub response_data: T,
}

impl<T: Serialize> APIResponse<T> {
    pub fn success(data: T) -> Json<Self> {
        Json(Self {
            response_message: "success".to_string(),
            response_data: data,
        })
    }
}

pub type AppResponse<T> = Result<Json<APIResponse<T>>, AppError>;
