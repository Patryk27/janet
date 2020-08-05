use crate::gitlab::{ProjectId, ProjectName};
use crate::interface::NamespacePtr;
use serde::Serialize;

mod parse;
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
