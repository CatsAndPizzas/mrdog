use std::error::Error;
#[macro_use]
extern crate prettytable;
mod cli;
mod mrdog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    cli::run().await
}
