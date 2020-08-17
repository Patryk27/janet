use crate::NamespacePtr;
use lib_gitlab::{ProjectId, ProjectName};
use serde::Serialize;

mod atom;
mod resolve;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ProjectPtr {
    Id(ProjectId),

    Name {
        namespace: Option<NamespacePtr>,
        name: ProjectName,
    },
}
