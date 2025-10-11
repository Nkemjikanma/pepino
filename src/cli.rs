use clap::{Parser, Subcommand};
use console::Style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

#[derive(Parser, Debug)]
#[command(name = "viter")]
#[command(about = "A Rust + Vite fullstack project scaffolder", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create new project
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
    Sqlx,
    Diesel,
}

pub fn init_cli() -> Result<Choices, dialoguer::Error> {
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
        .items(&["Axum", "Actix-web"])
        .default(0)
        .interact()?;

    let backend = match backend_index {
        0 => BackendFramework::Axum,
        1 => BackendFramework::ActixWeb,
        _ => {
            eprintln!("Backend framework not supported, using default");
            BackendFramework::Axum
        }
    };

    let database_index = Select::with_theme(&cli_theme)
        .with_prompt("Choose datase layer")
        .items(&["SQLx", "Diesel"])
        .default(0)
        .interact()?;

    let database = match database_index {
        0 => DatabaseLayer::Sqlx,
        1 => DatabaseLayer::Diesel,
        _ => {
            eprintln!("Database layer not supported, using default");
            DatabaseLayer::Sqlx
        }
    };

    Ok(Choices {
        project_name,
        backend,
        database,
    })
}

pub fn validate_project_name(input: &str) -> Result<(), &'static str> {
    if input.is_empty() {
        return Ok(());
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
