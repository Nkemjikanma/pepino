use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    #[typeshare(type = "string")]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[typeshare]
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}
