use std::fs;
use std::path::Path;

// create directory and parent directories
pub fn create_directory(path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

// write content to file
pub fn write_file(path: &Path, content: &[u8]) -> std::io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

// Replace {{PROJECT_NAME}} with the actual project name
pub fn replace_variable(template: &str, project_name: &str) -> String {
    template.replace("{{PROJECT_NAME}}", project_name)
}
