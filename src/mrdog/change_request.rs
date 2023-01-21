mod gitlab;

pub struct ChangeRequest {
    // TODO: fields
}

pub trait ChangeRequestProvider {
    // TODO: Maybe Future/Promise returned instead, not sure rn
    // TODO: Filter param + pagination (or rather limits + ordering)
    fn fetch(&self) -> Vec<ChangeRequest>;
}
