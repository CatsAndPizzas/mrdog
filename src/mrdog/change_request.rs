use async_trait::async_trait;

pub mod github;
pub mod gitlab;

#[derive(Debug)]
pub struct LinkedEntity {
    pub display_name: String,
    pub url: String,
}

impl LinkedEntity {
    pub fn new(display_name: String, url: String) -> Self {
        LinkedEntity { display_name, url }
    }
}

#[derive(Debug)]
pub struct ChangeRequest {
    pub id: String,
    pub url: String,
    pub title: String,
    pub project: LinkedEntity,
    pub assignees: Vec<LinkedEntity>,
    pub reviewers: Vec<LinkedEntity>,
}

#[async_trait]
pub trait ChangeRequestProvider {
    // TODO: Filter param + pagination (or rather limits + ordering)
    async fn fetch(&self) -> Vec<ChangeRequest>;
}
