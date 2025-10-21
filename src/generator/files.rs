use camino::Utf8Path;
use std::collections::HashMap;
use std::fs;

// create directory and parent directories
pub fn create_directory(path: &Utf8Path) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

// write content to file
pub fn write_file(path: &Utf8Path, content: &[u8]) -> std::io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

// Replace {{PROJECT_NAME}} with the actual project name
pub fn replace_variable(template: &str, project_name: &str) -> String {
    template.replace("{{PROJECT_NAME}}", project_name)
}

// merge toml files
pub fn merged_cargo_files(
    base_cargo: &str,
    server_fragment: &str,
    database_fragment: &str,
) -> String {
    let mut merged_cargo = String::new();

    // Base content (has [workspace] and base [dependencies])
    merged_cargo.push_str(base_cargo);
    merged_cargo.push('\n');

    // Append server dependencies
    merged_cargo.push_str(server_fragment);
    merged_cargo.push('\n');

    // Append database dependencies
    merged_cargo.push_str(database_fragment);
    merged_cargo.push('\n');

    merged_cargo
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_replace_variable() {
//         let template_cargo = r#"
// [package]
// name = "{{PROJECT_NAME}}"
// version = "0.1.0"
//         "#;
//
//         let template_package_json = r#"
//   "name": "{{PROJECT_NAME}}-client",
//   "version": "0.0.0"
//         "#;
//
//         let result_cargo = replace_variable(template_cargo, "myapp";
//         let result_package_json = replace_variable(template_package_json, "myapp");
//
//         assert!(result_cargo.contains("myapp"));
//         assert!(!result_cargo.contains("{{PROJECT_NAME}}"));
//
//         assert!(result_package_json.contains("myapp-client"));
//         assert!(!result_package_json.contains("{{PROJECT_NAME}}"));
//     }
// }
