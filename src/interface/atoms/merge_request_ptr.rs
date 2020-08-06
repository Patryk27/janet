use crate::gitlab::MergeRequestIid;
use crate::interface::ProjectPtr;
use serde::Serialize;
use url::Url;

mod parse;
mod resolve;

/// A reference to a merge request - e.g.: `some-project!123`.
///
/// Exposes a `.resolve()` method allowing to translate reference into a
/// specific id.
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
