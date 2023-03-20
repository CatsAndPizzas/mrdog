use async_trait::async_trait;

pub mod github;
pub mod gitlab;

#[derive(Debug)]
pub struct ChangeRequest {
    pub id: String,
    pub url: String,
    pub title: String,
}

#[async_trait]
pub trait ChangeRequestProvider {
    // TODO: Filter param + pagination (or rather limits + ordering)
    async fn fetch(&self) -> Vec<ChangeRequest>;
}
