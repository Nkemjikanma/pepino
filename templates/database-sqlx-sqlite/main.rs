mod common;
mod config;
mod db;
mod handlers;
mod models;

use sqlx::SqlitePool;
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
    pub pool: SqlitePool,
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

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    let app_state = AppState { pool };

    println!("migration: {:?}", migration_results);

    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, ORIGIN]);

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

    let host = config.server.host;
    let mut port: u16 = config.server.port.parse::<u16>().expect("invalid number");
    let server_address = format!("{}:{:?}", host, port);

    let listener = loop {
        match tokio::net::TcpListener::bind(&server_address).await {
            Ok(listener) => {
                println!("✅ Listening on http://{}", server_address);
                break listener;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                port += 1;
                println!("Port {} in use, trying {:?}", config.server.port, port);
                continue;
            }
            Err(_e) => (),
        }
    };

    tracing::info!("Server has started");

    axum::serve(listener, app).await?;

    Ok(())
}
