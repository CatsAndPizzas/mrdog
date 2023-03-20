use async_trait::async_trait;

pub mod github;
pub mod gitlab;

#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove when printed
pub struct ChangeRequest {
    id: String,
    url: String,
    title: String,
}

#[async_trait]
pub trait ChangeRequestProvider {
    // TODO: Filter param + pagination (or rather limits + ordering)
    async fn fetch(&self) -> Vec<ChangeRequest>;
}
