use std::error::Error;

mod cli;
mod mrdog;

fn main() -> Result<(), Box<dyn Error>> {
    cli::run()?;
    Ok(())
}
