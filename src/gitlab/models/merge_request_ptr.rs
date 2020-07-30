use crate::gitlab::{GitLabClient, MergeRequestId, ProjectId, ProjectPtr};
use anyhow::Result;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum MergeRequestPtr {
    Id {
        project: ProjectPtr,
        merge_request: MergeRequestId,
    },

    Url(String),
}

impl MergeRequestPtr {
    pub fn resolve(&self, client: &GitLabClient) -> Result<(ProjectId, MergeRequestId)> {
        todo!()
    }
}
