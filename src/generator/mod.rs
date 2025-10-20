pub mod files;
pub mod templates;

use crate::cli::Choices;
// use std::fs;
use std::path::Path;

pub fn generate_template(choices: Choices) -> Result<(), Box<dyn std::error::Error>> {
    // 2. Create root directory
    let project_name = choices.project_name;
    let project_root = Path::new(&project_name);

    // Validate project name
    if project_root.exists() {
        return Err("Directory already exists. Please enter a different name".into());
    }

    // Create project root directory
    println!("📁 Creating directory structure...");
    files::create_directory(project_root)?;

    // iterating through all embeds
    println!("📝 Generating files...");
    for file_path in templates::Templates::iter() {
        let file_path_str = file_path.as_ref();

        // remove template extensions
        let target_file_name = if file_path_str.ends_with(".template") {
            file_path_str.strip_suffix(".template").unwrap()
        } else {
            file_path_str
        };
        // get file content
        let file_content_as_bytes = templates::Templates::get(&file_path)
            .ok_or_else(|| format!("Failed to get embedded file: {}", file_path))?;

        // convert bytes to string for text files
        let file_content = match std::str::from_utf8(file_content_as_bytes.data.as_ref()) {
            Ok(text) => files::replace_variable(text, &project_name).into_bytes(),
            Err(_) => file_content_as_bytes.data.to_vec(),
        };

        // target path
        let target_path = project_root.join(target_file_name);

        // create parent path if needed
        if let Some(parent) = target_path.parent() {
            files::create_directory(parent)?;
        }

        files::write_file(&target_path, file_content.as_ref())
            .map_err(|e| format!("Failed to write file {}: {}", target_path.display(), e))?;
    }
    println!("✨ Finalizing project...");

    println!("\n✅ Project '{}' created successfully!", project_name);
    println!("\nNext steps:");
    println!("  cd {}", project_name);
    println!("  cp .env.example .env");
    println!("  # Edit .env with your database credentials");
    println!("  just migrate");
    println!("  just generate-types");
    println!("  just dev-server  # In one terminal");
    println!("  just dev-client  # In another terminal");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{BackendFramework, Choices, DatabaseLayer};
    use std::fs;
    use tempfile::TempDir;

    // handle the creation and cleanup of temporaryu dirs
    fn directory_manager<F, R>(f: F) -> R
    where
        F: FnOnce(&TempDir) -> R,
    {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let result = f(&temp_dir);

        std::env::set_current_dir(original_dir).unwrap();

        result
    }

    #[test]
    fn test_generate_project_directory() {
        directory_manager(|temp_dir| {
            let project_name = "new_pepino_project";

            let choices = Choices {
                project_name: project_name.to_string(),
                backend: BackendFramework::Axum,
                database: DatabaseLayer::Sqlx,
            };

            let result = generate_template(choices);

            assert!(result.is_ok());
            assert!(temp_dir.path().join(project_name).exists());
        });
    }

    #[test]
    fn test_generate_fails_if_direct_exists() {
        directory_manager(|_temp_dir| {
            let project_name = "new_pepino_project_failure";

            fs::create_dir(project_name).unwrap();

            let choices = Choices {
                project_name: project_name.to_string(),
                backend: BackendFramework::Axum,
                database: DatabaseLayer::Sqlx,
            };

            let result = generate_template(choices);
            // Should fail because directory exists
            assert!(result.is_err(), "Expected error but got: {:?}", result);
            assert!(result.unwrap_err().to_string().contains("already exists"));
        });
    }
}
