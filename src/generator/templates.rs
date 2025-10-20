use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates/"]
#[exclude = "target/*"]
#[exclude = "*/node_modules/*"]
#[exclude = "*/Cargo.lock"]
#[exclude = "*/.env"]
#[exclude = "*/.DS_Store"]
pub struct Templates;

// Workspace root files;
// pub const WORKSPACE_CARGO_TOML: &str = include_str!("../../template/Cargo.toml");
// pub const WORKSPACE_GIT_IGNORE: &str = include_str!("../../template/.gitignore");
// pub const WORKSPACE_JUSTFILE: &str = include_str!("../../template/justfile");
// pub const WORKSPACE_README: &str = include_str!("../../template/README.md");
//
// // Server files
// pub const SERVER_CARGO_TOML: &str = include_str!("../../template/server/Cargo.toml");
// pub const SERVER_ENV_EXAMPLE: &str = include_str!("../../template/server/.env.example");
// pub const SERVER_MAIN: &str = include_str!("../../template/server/src/main.rs");
// pub const SERVER_CONFIG: &str = include_str!("../../template/server/src/config.rs");
// pub const SERVER_DB: &str = include_str!("../../template/server/src/db.rs");
//
// pub const SERVER_COMMON_API: &str = include_str!("../../template/server/src/common/api.rs");
// pub const SERVER_COMMON_ERROR: &str = include_str!("../../template/server/src/common/errors.rs");
// pub const SERVER_COMMON_MOD: &str = include_str!("../../template/server/src/common/mod.rs");
//
// pub const SERVER_HANDLERS_PUBLIC: &str =
//     include_str!("../../template/server/src/handlers/public.rs");
// pub const SERVER_HANDLERS_MOD: &str = include_str!("../../template/server/src/handlers/mod.rs");
//
// pub const SERVER_MODELS_USERS: &str = include_str!("../../template/server/src/models/user.rs");
// pub const SERVER_MODELS_MOD: &str = include_str!("../../template/server/src/models/mod.rs");
//
// pub const SERVER_MIGRATION: &str =
//     include_str!("../../template/server/migrations/20251017202040_create_users_table.sql");
//
// // client files
// pub const CLIENT_PACKAGE_JSON: &str = include_str!("../../template/client/package.json");
// pub const CLIENT_VITE_CONFIG: &str = include_str!("../../template/client/vite.config.ts");
// pub const CLIENT_TS_CONFIG: &str = include_str!("../../template/client/tsconfig.json");
// pub const CLIENT_TS_CONFIG_NODE: &str = include_str!("../../template/client/tsconfig.node.json");
// pub const CLIENT_TS_CONFIG_APP: &str = include_str!("../../template/client/tsconfig.app.json");
// pub const CLIENT_INDEX_HTML: &str = include_str!("../../template/client/index.html");
// pub const CLIENT_GIT_INGNORE: &str = include_str!("../../template/client/.gitignore");
//
// pub const CLIENT_API: &str = include_str!("../../template/client/src/api/users.ts");
// pub const CLIENT_TYPES: &str = include_str!("../../template/client/src/types/api.ts");
//
// pub const CLINET_APP_TSX: &str = include_str!("../../template/client/src/App.tsx");
// pub const CLIENT_APP_CSS: &str = include_str!("../../template/client/src/App.css");
// pub const CLIENT_MAIN: &str = include_str!("../../template/client/src/main.tsx");
// pub const CLIENT_INDEX_CSS: &str = include_str!("../../template/client/src/index.css");
