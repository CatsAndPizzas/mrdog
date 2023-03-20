use std::error::Error;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use self::change_request::github::GitHubProvider;
use self::change_request::ChangeRequestProvider;

mod change_request;
mod config_storage;

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
    Ok(config_storage::set_config(item).map_err(|_| "sad")?)
}

pub async fn list() -> Result<(), Box<dyn Error>> {
    let mut change_requests = vec![];
    for p in initialize_providers() {
        change_requests.append(&mut p.fetch().await);
    }
    if change_requests.is_empty() {
        todo!("Print help")
    } else {
        println!("{change_requests:#?}");
        todo!("Print list")
    }
}

fn initialize_providers() -> Vec<Box<dyn ChangeRequestProvider>> {
    let mut vec: Vec<Box<dyn ChangeRequestProvider>> = vec![];
    for cf in &config_storage::get_tokens() {
        if let ConfigValue {
            name,
            value,
            provider: Provider::Github,
        } = cf
        {
            println!("Initializing GH {name}");
            vec.push(Box::new(GitHubProvider::new(value)));
        };
    }
    vec
}
