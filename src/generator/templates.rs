use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates/"]
#[exclude = "target/*"]
#[exclude = "*/node_modules/*"]
#[exclude = "*/Cargo.lock"]
#[exclude = "*/.env"]
#[exclude = "*/.DS_Store"]
pub struct Templates;
