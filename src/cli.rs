use clap::{Parser, Subcommand, ValueEnum};

use crate::mrdog;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Set config value
    Set { key: ConfigValue, value: String },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ConfigValue {
    Gitlab,
    Github,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub fn run() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Set { key, value }) => match key {
            ConfigValue::Github => mrdog::set_config(mrdog::ConfigItem::GitHubApiToken(value)),
            ConfigValue::Gitlab => mrdog::set_config(mrdog::ConfigItem::GitLabApiToken(value)),
        },
        None => mrdog::list(),
    }
}
