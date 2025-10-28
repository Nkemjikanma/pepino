mod cli;
use orchestra::{errors::PepinoError, models::PepinoProcess};

fn main() -> Result<(), PepinoError> {
    println!("Welcome to Pepino");

    let init_choices = cli::init_cli()?;

    if let PepinoProcess::Create { choices } = init_choices {
        println!(
            "Ready to create: {}, using {:?} for backend and {:?} for DB",
            choices.project_name, choices.backend, choices.database,
        );
        generator::generate_template(choices)?;
    };

    Ok(())
}
