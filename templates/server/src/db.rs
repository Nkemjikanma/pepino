use super::config::DBConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[tracing::instrument(name = "pool", skip_all)]
pub async fn create_pool(config: DBConfig) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Creating database pool connection");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .idle_timeout(config.idle_time)
        .acquire_timeout(config.connection_timeout)
        .connect(&config.url)
        .await?;

    Ok(pool)
}
