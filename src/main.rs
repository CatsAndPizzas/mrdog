use std::error::Error;
mod cli;
mod mrdog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    cli::run().await
}
