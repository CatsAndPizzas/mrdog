use std::{error::Error, io};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use self::change_request::{ChangeRequest, ChangeRequestProvider};

mod change_request;
mod config_storage;

// #[derive(Debug)]
// pub enum ConfigItem {
//     GitLabApiToken(String),
//     GitHubApiToken(String),
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize, Serialize)]
pub enum Provider {
    Gitlab,
    Github,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigValue {
    pub name: String,
    pub value: String,
    pub provider: Provider,
}

pub fn set_config(item: ConfigValue) -> Result<(), Box<dyn Error>> {
    let c = config_storage::set_config(item).map_err(|_| "sad")?;
    println!("{c:?}");
    Ok(())
}

pub fn list() -> Result<(), Box<dyn Error>> {
    let change_requests = initialize_providers()
        .iter()
        .flat_map(|p| p.fetch())
        .collect::<Vec<ChangeRequest>>();
    if change_requests.is_empty() {
        todo!("Print help")
    } else {
        todo!("Print list")
    }
}

fn initialize_providers() -> Vec<Box<dyn ChangeRequestProvider>> {
    // TODO: Read config and initialize all providers
    todo!("initialize_providers")
}
