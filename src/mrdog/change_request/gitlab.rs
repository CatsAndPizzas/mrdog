use async_trait::async_trait;

use super::ChangeRequestProvider;

struct GitLabProvider {}

#[async_trait]
impl ChangeRequestProvider for GitLabProvider {
    async fn fetch(&self) -> Vec<super::ChangeRequest> {
        todo!()
    }
}
