use crate::gitlab::{GitLabClient, ProjectId, ProjectName};
use anyhow::Result;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum ProjectPtr {
    Id(ProjectId),
    Name(ProjectName),
}

impl ProjectPtr {
    pub fn resolve(&self, client: &GitLabClient) -> Result<ProjectId> {
        todo!()
    }
}
