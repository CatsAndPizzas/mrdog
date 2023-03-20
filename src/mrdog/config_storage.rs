use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{self, PathBuf},
};

use super::{ConfigValue, Provider};

#[derive(Debug)]
pub enum ConfigError {
    CreateFailed(io::Error),
    CouldNotConvertToToml(toml::ser::Error),
    WriteToConfigFailed(io::Error),
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    github: Option<Vec<ConfigValue>>,
    gitlab: Option<Vec<ConfigValue>>,
}

fn update_or_push_config_value(values: &mut Vec<ConfigValue>, item: &ConfigValue) {
    for value in values.iter_mut() {
        if value.name == item.name {
            value.value = item.value.clone();
            return;
        }
    }
    values.push(item.clone());
}

pub fn set_config(item: ConfigValue) -> Result<(), ConfigError> {
    let mut config = load_or_create_config();

    match item.provider {
        Provider::Gitlab => {
            if let Some(values) = &mut config.gitlab {
                update_or_push_config_value(values, &item);
            } else {
                config.gitlab = Some(vec![item]);
            }
        }
        Provider::Github => {
            if let Some(values) = &mut config.github {
                update_or_push_config_value(values, &item);
            } else {
                config.github = Some(vec![item]);
            }
        }
    }

    let to_set = toml::to_string(&config).map_err(ConfigError::CouldNotConvertToToml)?;

    fs::write(get_config_path(), to_set.as_bytes()).map_err(ConfigError::WriteToConfigFailed)?;

    Ok(())
}

pub fn get_tokens() -> Vec<ConfigValue> {
    // TODO: Get all tokens
    load_or_create_config().github.unwrap_or(vec![])
}

fn load_or_create_config() -> Config {
    let exists = path::Path::new(&get_config_path()).exists();

    if !exists {
        println!("Config does not exists, creating...");
        let created_config = create_config();
        if created_config.is_ok() {
            println!("Created config ~/.config/mrdog.toml");
        } else {
            println!("Could not create a config, aborting, {created_config:?}");
            panic!();
        }
    }

    let contents = fs::read_to_string(get_config_path()).unwrap();
    let tokens: Config = toml::from_str(&contents).unwrap();
    tokens
}

fn create_config() -> Result<bool, ConfigError> {
    let written = fs::File::create(get_config_path());
    written.map(|_| true).map_err(ConfigError::CreateFailed)
}

fn get_config_path() -> PathBuf {
    let home = dirs::home_dir();
    if let Some(p) = home {
        p.join(".config/mrdog.toml")
    } else {
        panic!("Home directory not found!");
    }
}
