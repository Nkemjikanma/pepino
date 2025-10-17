use crate::AppState;
use crate::common::{APIResponse, AppResponse};
use serde::{Deserialize, Serialize};

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

use sqlx::FromRow;
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("users", get(list_users).post(create_user))
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: String,
}
pub async fn health(State(state): State<AppState>) -> AppResponse<HealthStatus> {
    sqlx::query("SELECT 1").execute(&state.pool).await?;

    Ok(APIResponse::success(HealthStatus {
        status: "ok".to_string(),
        database: "connected".to_string(),
    }))
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

pub async fn list_users(State(state): State<AppState>) -> AppResponse<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT id, email, name, created_at FROM users")
        .fetch_all(&state.pool)
        .await?;

    Ok(APIResponse::success(users))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> AppResponse<User> {
    let user = sqlx::query_as::<_, User>("INSERT INTO users (email, namne, created_at) VALUES ($1, $2, NOW()) RETURNING id, email, name, created_at").bind(&request.email).bind(&request.name).fetch_one(&state.pool).await?;

    Ok(APIResponse::success(user))
}
