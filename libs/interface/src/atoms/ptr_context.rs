use lib_gitlab::{NamespaceId, ProjectId};
use serde::Serialize;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize)]
pub struct PtrContext {
    pub namespace_id: Option<NamespaceId>,
    pub project_id: Option<ProjectId>,
}
