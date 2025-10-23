use super::error::PepinoError;
use clap::{Parser, Subcommand};
use console::Style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

#[derive(Parser, Debug)]
#[command(name = "pepino")]
#[command(author = "Nkemjika")]
#[command(version = "0.1.0")]
#[command(about = "🥒 A fullstack Rust + Vite project scaffolder", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create new project
    #[command(name = "new")]
    New { name: Option<String> },
}

#[derive(Debug)]
pub struct Choices {
    pub project_name: String,
    pub backend: BackendFramework,
    pub database: DatabaseLayer,
}

#[derive(Debug)]
pub enum BackendFramework {
    Axum,
    ActixWeb,
}

#[derive(Debug)]
pub enum DatabaseLayer {
    Sqlx(SQLXFlavour),
    // Diesel,
}

#[derive(Debug)]
pub enum SQLXFlavour {
    PostgreSQL { docker_compose: bool },
    SQLite,
}

pub fn init_cli() -> Result<Choices, PepinoError> {
    let cli = Cli::parse();

    let cli_theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    // if !Confirm::with_theme(&cli_theme)
    //     .with_prompt("Do you want to continue?")
    //     .interact()?
    // {
    //     return Ok(None);
    // }

    let cli_name = match cli.command {
        Commands::New { name } => name,
    };

    let validated_name = if let Some(name) = cli_name {
        match validate_project_name(&name) {
            Ok(_) => {
                println!("✓ Using project name: {}", name);
                Some(name)
            }
            Err(e) => {
                eprintln!("Invalid name '{}': {}", name, e);
                None
            }
        }
    } else {
        None
    };

    //Interactive node
    let project_name = if let Some(name) = validated_name {
        println!("✅ Project name: {}", name);
        name
    } else {
        Input::<String>::with_theme(&cli_theme)
            .with_prompt("Enter project name")
            .validate_with(|input: &String| validate_project_name(&input))
            .interact_text()?
    };

    let backend_index = Select::with_theme(&cli_theme)
        .with_prompt("Choose backend framework")
        .items(&["Axum"])
        .default(0)
        .interact()?;

    let backend = match backend_index {
        0 => BackendFramework::Axum,
        // 1 => BackendFramework::ActixWeb,
        _ => unreachable!("Backend framework not supported, using default"),
    };

    let database_index = Select::with_theme(&cli_theme)
        .with_prompt("Choose database layer")
        .items(&["SQLx (async, supports Postgres/SQLite)"])
        .default(0)
        .interact()?;

    let database = match database_index {
        0 => {
            let arrow = Style::new().cyan().bold();
            let sub_style = Style::new().dim();
            // Informative message  and indentation
            println!(
                "\n{} {}",
                arrow.apply_to("→"),
                sub_style.apply_to("SQLx selected — choose which flavour to use:")
            );

            let sqlx_flavours = ["PostgreSQL", "SQLite"];
            let flavour_index = Select::with_theme(&cli_theme)
                .with_prompt("Choose SQLx flavour")
                .items(
                    &sqlx_flavours
                        .iter()
                        .map(|f| format!("   {}", f))
                        .collect::<Vec<_>>(),
                )
                .default(0)
                .interact()?;

            let flavour = match flavour_index {
                0 => {
                    let docker_compose = Confirm::with_theme(&cli_theme)
                        .with_prompt("Include docker-compose for database?")
                        .default(true)
                        .show_default(false)
                        .wait_for_newline(true)
                        .interact()?;

                    if docker_compose {
                        SQLXFlavour::PostgreSQL {
                            docker_compose: true,
                        }
                    } else {
                        SQLXFlavour::PostgreSQL {
                            docker_compose: false,
                        }
                    }
                }
                1 => SQLXFlavour::SQLite,
                _ => unreachable!("SQLx flavour not supported, using default"),
            };
            DatabaseLayer::Sqlx(flavour)
        }
        // 1 => DatabaseLayer::Diesel,
        _ => unreachable!("Database layer not supported, using default"),
    };

    Ok(Choices {
        project_name,
        backend,
        database,
    })
}

pub fn validate_project_name(input: &str) -> Result<(), &'static str> {
    if input.is_empty() {
        return Err("project name can't be empty");
    }

    let first_char = input.chars().next().unwrap();

    if first_char.is_numeric() {
        return Err("project name can't start with a number");
    } else if !first_char.is_ascii_lowercase() {
        return Err("project name must start with lowercase");
    }

    for ch in input.chars().skip(1) {
        if !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '-' {
            return Err("project name can contain only lowercase letters, numbers and ");
        }
    }
    Ok(())
}
