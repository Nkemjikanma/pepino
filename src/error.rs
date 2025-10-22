use thiserror::Error;

#[derive(Debug, Error)]
pub enum PepinoError {
    #[error("Project directory '{0}' already exists")]
    DirectoryExists(String),

    #[error("Failed to read template file: {0}")]
    MissingTemplate(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("CLI interaction error: {0}")]
    Dialoguer(#[from] dialoguer::Error),

    #[error("Failed to create directory: {0}")]
    CreateDirectory(String),

    #[error("Failed to write file '{path}': {source}")]
    WriteFile {
        path: String,
        source: std::io::Error,
    },

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
