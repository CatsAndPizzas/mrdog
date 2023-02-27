use std::future::Future;

use async_trait::async_trait;
use octocrab::Octocrab;

use super::ChangeRequestProvider;

pub struct GitHubProvider {
    token: String,
}

impl GitHubProvider {
    pub fn new(token: &str) -> GitHubProvider {
        GitHubProvider {
            token: token.to_string(),
        }
    }
}

#[async_trait]
impl ChangeRequestProvider for GitHubProvider {
    async fn fetch(&self) -> Vec<super::ChangeRequest> {
        let instance = Octocrab::builder()
            .personal_token(self.token.clone())
            .build()
            .unwrap();
        let mrs = instance
            .search()
            // TODO: assignee, mentions, review-requested
            .issues_and_pull_requests("is:open is:pr author:@me archived:false")
            .send()
            .await
            .expect("Request succeeds"); // TODO: Result
        mrs.into_iter()
            .map(|i| super::ChangeRequest {
                id: i.id.to_string(),
                url: i.url.into(),
                title: i.title,
            })
            .collect()
    }
}
