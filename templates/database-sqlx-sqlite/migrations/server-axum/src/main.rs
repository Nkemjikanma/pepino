mod common;
mod config;
mod db;
mod handlers;
mod models;

use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::config::Config;
use crate::db::create_pool;
use crate::handlers::public::public_routes;
use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    },
};
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = Config::default().map_err(|e| format!("Config error: {:?}", e))?;
    start_server(config).await?;

    Ok(())
}

async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing DB connection");

    let pool = create_pool(config.db).await?;
    let app_state = AppState { pool };

    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, ORIGIN]);

    tracing::info!("Setting up routes");

    // let protected_api = Router::new()
    //     .merge(auth_routes())
    //     .route_layer(middleware::from_fn_with_state(app_state.clone()));

    let public_api = Router::new().merge(public_routes());

    let app = Router::new()
        .nest("/api", public_api)
        .layer(cors)
        .with_state(app_state)
        .into_make_service_with_connect_info::<SocketAddr>();

    let server_address = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    tracing::info!("Server has started");

    axum::serve(listener, app).await?;

    Ok(())
}
