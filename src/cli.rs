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
}
pub fn init_cli() -> Result<Option<Choices>, Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    if !Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        return Ok(None);
    }

    match cli.command {
        Commands::New { name } => {
            let project_name = if let Some(n) = name {
                n
            } else {
                Input::<String>::with_theme(&theme)
                    .with_prompt("Enter project name")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.chars().next().unwrap().is_numeric() {
                            Err("project name can't start with a number")
                        } else if !input.chars().next().unwrap().is_ascii_lowercase() {
                            Err("project name must start with lowercase")
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()?
            };
            println!("Creating project: {:?}", project_name);

            Ok(Some(Choices { project_name }))
        }
    }
}
