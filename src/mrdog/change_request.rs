use async_trait::async_trait;

pub mod github;
pub mod gitlab;

#[derive(Debug)]
pub struct ChangeRequest {
    id: String,
    url: String,
    title: String,
}

#[async_trait]
pub trait ChangeRequestProvider {
    // TODO: Maybe Future/Promise returned instead, not sure rn
    // TODO: Filter param + pagination (or rather limits + ordering)
    async fn fetch(&self) -> Vec<ChangeRequest>;
}
