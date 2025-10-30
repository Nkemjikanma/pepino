mod cli;
mod error;
mod generator;

fn main() -> Result<(), error::PepinoError> {
    println!("Welcome to Pepino");

    let choices = cli::init_cli()?;

    println!(
        "Ready to create: {}, using {:?} for backend and {:?} for DB",
        choices.project_name, choices.backend, choices.database,
    );

    generator::generate_template(choices)?;
    Ok(())
}
