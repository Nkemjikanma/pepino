mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Viter");

    let choices = cli::init_cli()?;

    println!(
        "Ready to create: {}, using {:?} for backend and {:?} for DB",
        choices.project_name, choices.backend, choices.database,
    );

    Ok(())
}
