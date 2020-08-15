use crate::ProjectPtr;
use lib_gitlab::MergeRequestIid;
use serde::Serialize;
use url::Url;

mod atom;
mod resolve;

/// A reference to a merge request, e.g. `some-project!123`.
///
/// This structure exposes a `.resolve()` function that allows to transform it
/// into specific project & merge request ids.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum MergeRequestPtr {
    /// E.g. `!123` or `foo!123`
    Iid {
        project: Option<ProjectPtr>,
        merge_request: MergeRequestIid,
    },

    /// E.g. `https://gitlab.com/repository/project/-/merge_requests/123`
    Url(Url),
}
