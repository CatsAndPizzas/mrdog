use super::ChangeRequestProvider;

struct GitLabProvider {}

impl ChangeRequestProvider for GitLabProvider {
    fn fetch(&self) -> Vec<super::ChangeRequest> {
        todo!()
    }
}
