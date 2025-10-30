use super::config::DBConfig;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

use std::str::FromStr;

#[tracing::instrument(name = "pool", skip_all)]
pub async fn create_pool(config: DBConfig) -> Result<SqlitePool, sqlx::Error> {
    tracing::info!("Creating database pool connection");

    let options = SqliteConnectOptions::from_str(&config.url)?.create_if_missing(true); // This is the only SQLite-specific thing you need

    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .idle_timeout(config.idle_time)
        .acquire_timeout(config.connection_timeout)
        .connect_with(options)
        .await?;

    Ok(pool)
}
