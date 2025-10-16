use crate::AppState;
use crate::common::api::{APIResponse, AppResponse};
use crate::services::user::create_user;
use serde::{Deserialize, Serialize};

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("signup", post(signup_handler))
}

pub async fn root() -> &'static str {
    "Welcome to Entry"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupPayload {
    pub email: String,
    pub password: String,
}

pub async fn signup_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SignupPayload>,
) -> AppResponse<i64> {
    tracing::info!("Creating user");

    let user_id = create_user(&app_state, payload).await?;

    tracing::info!("User signup successful");

    Ok(APIResponse::success(user_id))
}
