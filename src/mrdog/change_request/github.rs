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

// Test @ https://docs.github.com/en/graphql/overview/explorer
const QUERY: &str = "
{
    viewer {
      pullRequests(first: 100, states: OPEN) {
        nodes {
          id
          title
          url
          author {
            login
          }
          assignees(first: 100) {
            nodes {
              login
            }
          }
          reviews(first: 100) {
            edges {
              node {
                id
              }
            }
          }
          changedFiles
          additions
          deletions
          createdAt
          reviewDecision
          headRepository {
            name
            url
          }
        }
      }
    }
  }
";

#[async_trait]
impl ChangeRequestProvider for GitHubProvider {
    async fn fetch(&self) -> Vec<super::ChangeRequest> {
        let instance = Octocrab::builder()
            .personal_token(self.token.clone())
            .build()
            .unwrap();

        let response: serde_json::Value = instance.graphql(QUERY).await.unwrap();
        // TODO: Result
        response["data"]["viewer"]["pullRequests"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| super::ChangeRequest {
                id: i["id"].as_str().unwrap().to_string(),
                url: i["url"].as_str().unwrap().to_string(),
                title: i["title"].as_str().unwrap().to_string(),
            })
            .collect()
    }
}
