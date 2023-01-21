use super::ChangeRequestProvider;

struct GitHubProvider {}

impl ChangeRequestProvider for GitHubProvider {
    fn fetch(&self) -> Vec<super::ChangeRequest> {
        todo!()
    }
}
