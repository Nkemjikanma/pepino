pub mod files;
pub mod templates;

use crate::error::PepinoError;

use crate::cli::{Choices, DatabaseLayer, SQLXFlavour};
use camino::{Utf8Path, Utf8PathBuf};
// use std::fs;
use std::collections::HashMap;

pub enum SQLXTemplates {
    Postgres(templates::PostgresTemplates),
    Sqlite(templates::SqliteTemplates),
}

impl SQLXTemplates {
    pub fn get(&self, path: &str) -> Option<rust_embed::EmbeddedFile> {
        match self {
            SQLXTemplates::Postgres(_) => templates::PostgresTemplates::get(path),
            SQLXTemplates::Sqlite(_) => templates::SqliteTemplates::get(path),
        }
    }
}

pub fn generate_template(choices: Choices) -> Result<(), PepinoError> {
    // extract project details
    let project_name = choices.project_name;

    let (database_flavour, include_docker_compose) = match choices.database {
        DatabaseLayer::Sqlx(flavour) => match flavour {
            SQLXFlavour::PostgreSQL { docker_compose } => (
                SQLXTemplates::Postgres(templates::PostgresTemplates),
                docker_compose,
            ),
            SQLXFlavour::SQLite => (SQLXTemplates::Sqlite(templates::SqliteTemplates), false),
        },
    };

    let project_root = Utf8Path::new(&project_name);

    // Validate project name
    if project_root.exists() {
        return Err(PepinoError::DirectoryExists(project_name));
    }

    // Create project root directory
    println!("📁 Creating directory structure...");
    files::create_directory(&project_root)?;

    // iterating through all embeds
    println!("📝 Generating files...");
    let mut files_to_write = HashMap::new();

    for file_path in templates::BaseTemplates::iter() {
        let file_path_str = file_path.as_ref();

        let target_path = project_root.join(file_path_str);

        let content = templates::BaseTemplates::get(file_path_str)
            .ok_or_else(|| PepinoError::MissingTemplate(file_path_str.to_string()))?;

        files_to_write.insert(target_path, content);
    }

    for file_path in templates::ClientTemplates::iter() {
        let file_path_str = file_path.as_ref();

        let target_path = project_root.join("client").join(file_path_str);

        let content = templates::ClientTemplates::get(&file_path_str)
            .ok_or_else(|| PepinoError::MissingTemplate(file_path_str.to_string()))?;

        files_to_write.insert(target_path, content);
    }

    for file_path in templates::ServerTemplates::iter() {
        let file_path_str = file_path.as_ref();

        let target_path = project_root.join("server").join(file_path_str);

        let content = templates::ServerTemplates::get(file_path_str)
            .ok_or_else(|| PepinoError::MissingTemplate(file_path_str.to_string()))?;

        files_to_write.insert(target_path, content);
    }

    for file_path in match &database_flavour {
        SQLXTemplates::Postgres(_) => templates::PostgresTemplates::iter().collect::<Vec<_>>(),
        SQLXTemplates::Sqlite(_) => templates::SqliteTemplates::iter().collect::<Vec<_>>(),
    } {
        let file_path_str = file_path.as_ref();

        let target_path = if file_path_str.starts_with("migrations/") {
            project_root.join("server").join(file_path_str)
        } else if file_path_str == "db.rs" {
            project_root.join("server/src").join(file_path_str)
        } else if file_path_str == "Cargo.fragment.toml" {
            continue;
        } else if file_path_str == ".env.example" {
            project_root.join(file_path_str)
        } else if file_path_str == "main.rs" {
            project_root.join("server/src").join(file_path_str)
        } else {
            project_root.join("server").join(file_path_str)
        };

        if file_path_str == "docker-compose.yml" && !include_docker_compose {
            continue;
        }

        let content = database_flavour
            .get(file_path_str)
            .ok_or_else(|| PepinoError::MissingTemplate(file_path_str.to_string()))?;

        files_to_write.insert(target_path, content);
    }

    let server_base = std::str::from_utf8(
        templates::ServerTemplates::get("Cargo.base.toml")
            .ok_or_else(|| PepinoError::MissingTemplate("Cargo.base.toml".to_string()))?
            .data
            .as_ref(),
    )?
    .to_string();

    let server_fragment = std::str::from_utf8(
        templates::ServerTemplates::get("Cargo.fragment.toml")
            .ok_or_else(|| PepinoError::MissingTemplate("Server Cargo.fragment.toml".to_string()))?
            .data
            .as_ref(),
    )?
    .to_string();

    let db_fragment = match database_flavour {
        SQLXTemplates::Postgres(_) => std::str::from_utf8(
            templates::PostgresTemplates::get("Cargo.fragment.toml")
                .ok_or_else(|| {
                    PepinoError::MissingTemplate(
                        "Database-Postgres Cargo.fragment.toml".to_string(),
                    )
                })?
                .data
                .as_ref(),
        )?
        .to_string(),
        SQLXTemplates::Sqlite(_) => std::str::from_utf8(
            templates::SqliteTemplates::get("Cargo.fragment.toml")
                .ok_or_else(|| {
                    PepinoError::MissingTemplate("Database-Sqlite Cargo.fragment.toml".to_string())
                })?
                .data
                .as_ref(),
        )?
        .to_string(),
    };

    let merged_cargo = files::merged_cargo_files(&server_base, &server_fragment, &db_fragment);

    let server_cargo_path = project_root.join("server/Cargo.toml");

    if let Some(parent) = server_cargo_path.parent() {
        files::create_directory(parent)?;
    }

    let cargo_with_vars = files::replace_variable(&merged_cargo, &project_name);
    files::write_file(&server_cargo_path, cargo_with_vars.as_bytes())?;

    for (path, content) in files_to_write {
        let path_str = path.as_str();

        // Do not write Cargo.fragment.toml to template
        if path_str.contains(".fragment.toml") || path_str.contains(".base.toml") {
            continue;
        }
        let final_path = if path_str.ends_with(".template") {
            Utf8PathBuf::from(path_str.strip_suffix(".template").unwrap())
        } else {
            path
        };

        // create parent dir
        if let Some(parent) = final_path.parent() {
            files::create_directory(parent)?;
        }

        // convert bytes to string for text files
        let file_content = match std::str::from_utf8(content.data.as_ref()) {
            Ok(text) => files::replace_variable(text, &project_name).into_bytes(),
            Err(_) => content.data.to_vec(), // Write binary file as is
        };

        files::write_file(&final_path, &file_content)?;
    }

    println!("✨ Finalizing project...");

    println!("\n✅ Project '{}' created successfully!", project_name);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::cli::{BackendFramework, Choices, DatabaseLayer};
//     use std::fs;
//     use tempfile::TempDir;
//
//     // handle the creation and cleanup of temporaryu dirs
//     fn directory_manager<F, R>(f: F) -> R
//     where
//         F: FnOnce(&TempDir) -> R,
//     {
//         let temp_dir = TempDir::new().unwrap();
//         let original_dir = std::env::current_dir().unwrap();
//
//         std::env::set_current_dir(&temp_dir).unwrap();
//
//         let result = f(&temp_dir);
//
//         std::env::set_current_dir(original_dir).unwrap();
//
//         result
//     }
//
//     #[test]
//     fn test_generate_project_directory() {
//         directory_manager(|temp_dir| {
//             let project_name = "new_pepino_project";
//
//             let choices = Choices {
//                 project_name: project_name.to_string(),
//                 backend: BackendFramework::Axum,
//                 database: DatabaseLayer::Sqlx,
//             };
//
//             let result = generate_template(choices);
//
//             assert!(result.is_ok());
//             assert!(temp_dir.path().join(project_name).exists());
//         });
//     }
//
//     #[test]
//     fn test_generate_fails_if_direct_exists() {
//         directory_manager(|_temp_dir| {
//             let project_name = "new_pepino_project_failure";
//
//             fs::create_dir(project_name).unwrap();
//
//             let choices = Choices {
//                 project_name: project_name.to_string(),
//                 backend: BackendFramework::Axum,
//                 database: DatabaseLayer::Sqlx,
//             };
//
//             let result = generate_template(choices);
//             // Should fail because directory exists
//             assert!(result.is_err(), "Expected error but got: {:?}", result);
//             assert!(result.unwrap_err().to_string().contains("already exists"));
//         });
//     }
// }
