use std::time::Duration;

pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_time: Duration,
}

pub struct Server {
    pub host: String,
    pub port: String,
}
pub struct Config {
    pub db: DBConfig,
    pub server: Server,
}

impl Config {
    pub fn default() -> Result<Self, ConfigError> {
        Ok(Self {
            db: DBConfig {
                url: std::env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnv)?,
                max_connections: 5,
                min_connections: 1,
                connection_timeout: Duration::from_secs(200),
                idle_time: Duration::from_secs(300),
            },
            server: Server {
                host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("PORT").unwrap_or_else(|_| "8000".to_string()),
            },
        })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv,
}
