use crate::common::{
    errors::AppError,
    utils::{AppState, PasswordUtils},
};
use crate::handlers::public::SignupPayload;

use chrono::Utc;

// TODO: Implement Error crate
pub async fn create_user(app_state: &AppState, request: SignupPayload) -> Result<i64, AppError> {
    let maybe_user: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(&request.email)
            .fetch_one(&app_state.pool)
            .await
            .map_err(|e| {
                tracing::error!("User already exists: {:?}", e);
                AppError::ErrorQueryingDatabase
            })?;

    if maybe_user {
        return Err(AppError::ExistingUser);
    }

    let mut begin_pool = app_state.pool.begin().await.map_err(|_| {
        tracing::error!("Error starting database for user sign up");
        AppError::ErrorQueryingDatabase
    })?;

    let hashed_password = PasswordUtils::hash_password(&request.password)?;

    let new_user = sqlx::query_scalar::<_, i64>(
        "INSERT INTO users (email, password_hash, created_at) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&request.email)
    .bind(&hashed_password)
    .bind(Utc::now().naive_utc())
    .fetch_one(&mut *begin_pool)
    .await
    .map_err(|e| {
        tracing::error!("Error creating user in database: {:?}", e);

        AppError::ErrorQueryingDatabase
    })?;

    begin_pool.commit().await.map_err(|e| {
        tracing::error!("Error commiting user transaction into DB {:?}", e);
        AppError::ErrorQueryingDatabase
    })?;

    Ok(new_user)
}
