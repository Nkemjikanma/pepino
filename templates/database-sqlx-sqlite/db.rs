use super::config::DBConfig;
use sqlx::Sqlite::{SqlitePool, SqlitePoolOptions};

#[tracing::instrument(name = "pool", skip_all)]
pub async fn create_pool(config: DBConfig) -> Result<SqlitePool, sqlx::Error> {
    tracing::info!("Creating database pool connection");

    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .idle_timeout(config.idle_time)
        .acquire_timeout(config.connection_timeout)
        .connect(&config.url)
        .await?;

    Ok(pool)
}
