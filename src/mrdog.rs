use self::change_request::{ChangeRequest, ChangeRequestProvider};

mod change_request;
mod config_storage;

#[derive(Debug)]
pub enum ConfigItem {
    GitLabApiToken(String),
    GitHubApiToken(String),
}

pub fn set_config(item: ConfigItem) {
    config_storage::set_config(item)
}

pub fn list() {
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
