mod cli;

use clap::{Parser, Subcommand};
use console::Style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    println!("Welcome to Viter");

    cli::init_cli();

    Ok(())
}
