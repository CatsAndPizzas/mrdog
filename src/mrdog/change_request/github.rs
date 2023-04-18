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
        number
        repository {
          nameWithOwner
          url
        }
        title
        url
        author {
          login
          url
        }
        assignees(first: 100) {
          nodes {
            login
            url
          }
        }
        reviews(first: 100) {
          nodes {
            id
            author {
              url
              login
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
                project: super::LinkedEntity::new(
                    i["repository"]["nameWithOwner"]
                        .as_str()
                        .unwrap_or("x")
                        .to_string(),
                    i["repository"]["url"].as_str().unwrap_or("x").to_string(),
                ),
                assignees: i["assignees"]["nodes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|a| {
                        super::LinkedEntity::new(
                            a["login"].as_str().unwrap().to_string(),
                            a["url"].as_str().unwrap().to_string(),
                        )
                    })
                    .collect(),
                reviewers: i["reviews"]["nodes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|r| {
                        super::LinkedEntity::new(
                            r["author"]["login"].as_str().unwrap().to_string(),
                            r["author"]["url"].as_str().unwrap().to_string(),
                        )
                    })
                    .collect(),
                id: format!("{}", i["number"].as_u64().unwrap_or(0)),
                url: i["url"].as_str().unwrap().to_string(),
                title: i["title"].as_str().unwrap().to_string(),
            })
            .collect()
    }
}
