// use super::error::PepinoError;

use orchestra::{errors::PepinoError, models::{PepinoProcess, Choices, BackendFramework, SQLXFlavour, DatabaseLayer, BuildProcess}};
use clap::{Parser, Subcommand};
use console::Style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

#[derive(Parser, Debug)]
#[command(name = "pepino")]
#[command(author = "Nkemjika")]
#[command(version = "0.1.0")]
#[command(about = "🥒 A fullstack Rust + Vite project scaffolder + Build System", long_about = None)]
pub struct Cli {
    /// Global flag available to all subcommands
    // #[arg(short, long, global = true)]
    // pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create new project
    #[command(name = "new")]
    New {
        #[arg(help = "Optional project name")]
        name: Option<String>,
    },

    /// Start development servers
    #[command(name = "dev")]
    Dev {
        #[arg(long, help = "Path to project")]
        project_path: Option<String>,
    }, // the directory of the project to run?

    #[command(name = "build")]
    Build {
        #[command(subcommand)]
        target: BuildTarget,
    }, // the directory of the project to build?
}

#[derive(Subcommand, Debug)]
pub enum BuildTarget {
    /// Build the frontend
    Frontend {
        #[arg(long, help = "Minify output")]
        release: bool,
    },
    /// Build the backend
    Backend {
        #[arg(long, help = "Target architecture")]
        target: Option<String>,
    },
}

pub fn init_cli() -> Result<PepinoProcess, PepinoError> {
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

    match cli.command {
        Commands::New { name } => Ok(PepinoProcess::Create {
            choices: { create_pepino_project(name, &cli_theme)? },
        }),
        Commands::Dev { project_path } => {
            if let Some(ref path) = project_path {
                println!("starting servers on {}", path);
            }
            Ok(PepinoProcess::Dev { path: project_path })
        }
        Commands::Build { target } => match target {
            BuildTarget::Frontend { release } => {
                println!("Building frontend (release={})", release);
                Ok(PepinoProcess::Build(BuildProcess::Frontend { release }))
            }
            BuildTarget::Backend { target } => {
                println!("Building backend for {:?}", target);
                Ok(PepinoProcess::Build(BuildProcess::Backend { target }))
            }
        },
    }
}

pub fn create_pepino_project(
    cli_name: Option<String>,
    cli_theme: &ColorfulTheme,
) -> Result<Choices, dialoguer::Error> {
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
        Input::<String>::with_theme(cli_theme)
            .with_prompt("Enter project name")
            .validate_with(|input: &String| validate_project_name(&input))
            .interact_text()?
    };

    let backends_list = ["Axum"];
    let backend_index = Select::with_theme(cli_theme)
        .with_prompt("Choose backend framework")
        .items(&backends_list)
        .default(0)
        .interact()?;

    let backend = match backend_index {
        0 => BackendFramework::Axum,
        // 1 => BackendFramework::ActixWeb,
        _ => unreachable!("Backend framework not supported, using default"),
    };

    let database_index = Select::with_theme(cli_theme)
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
            let flavour_index = Select::with_theme(cli_theme)
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
                    let docker_compose = Confirm::with_theme(cli_theme)
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
            return Err(
                "project name can contain only lowercase letters, numbers, and hyphens ('-')",
            );
        }
    }
    Ok(())
}
