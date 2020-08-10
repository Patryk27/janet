use crate::{Id, Project};

#[derive(Clone, Debug)]
pub struct NewMergeRequest {
    pub project_id: Id<Project>,
    pub ext_id: i64,
    pub iid: i64,
    pub state: String,
}
