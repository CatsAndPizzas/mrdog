use std::error::Error;

use clap::{Parser, Subcommand};

use crate::mrdog::{self, set_config, ConfigValue, Provider};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set config value
    Set {
        // TODO: allow shorthand values like gh / gl
        #[arg(long = "provider", default_value_t = Provider::Github, value_enum)]
        provider: Provider,
        #[arg(short = 'n', default_value_t = String::from("default"))]
        name: String,
        value: String,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Set {
            value,
            name,
            provider,
        }) => {
            let config_value = ConfigValue {
                value,
                name,
                provider,
            };
            println!("{config_value:?}");
            match provider {
                Provider::Gitlab => set_config(config_value),
                Provider::Github => set_config(config_value),
            }
        }
        None => mrdog::list().await,
    }
}
