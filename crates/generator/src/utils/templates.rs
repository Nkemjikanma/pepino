use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../../templates/base/"]
pub struct BaseTemplates;

#[derive(Embed)]
#[folder = "../../templates/client/"]
pub struct ClientTemplates;

#[derive(Embed)]
#[folder = "../../templates/server-axum/"]
pub struct ServerTemplates;

// Will be used based on database choice
#[derive(Embed)]
#[folder = "../../templates/database-sqlx-postgres/"]
pub struct PostgresTemplates;

#[derive(Embed)]
#[folder = "../../templates/database-sqlx-sqlite/"]
pub struct SqliteTemplates;
