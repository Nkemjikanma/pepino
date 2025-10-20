use std::fs;
use std::path::Path;

// create directory and parent directories
// Use fs::create_dir_all to create parent directories too
// What function creates directories?
pub fn create_directory(path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

// write content to file
// Write the bytes to the file
// What function writes files?
pub fn write_file(path: &Path, content: &[u8]) -> std::io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

// Replace {{PROJECT_NAME}} with the actual project name
// What method on &str replaces text?
pub fn replace_variable(template: &str, project_name: &str) -> String {
    template.replace("{{PROJECT_NAME}}", project_name)
}
