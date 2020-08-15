use lib_gitlab::{NamespaceId, ProjectId};
use serde::Serialize;

/// When writing notes (e.g. for merge requests or issues), GitLab allows to
/// omit things that can be inferred from the context - e.g. when you write
/// `!123`, GitLab will automatically infer that you actually meant
/// `$current-namespace/$current-project!123`.
///
/// Since we strive to be compatible with GitLab's formatting, we also support
/// this syntax - and we use this struct to carry the "inferring context" for
/// functions like `MergeRequestPtr::resolve()`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize)]
pub struct PtrContext {
    pub namespace_id: Option<NamespaceId>,
    pub project_id: Option<ProjectId>,
}
