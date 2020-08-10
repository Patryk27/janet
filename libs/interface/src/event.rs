use lib_gitlab::{MergeRequestIid, ProjectId};
use serde::Serialize;

/// A generic event accepted by Janet.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    MergeRequestClosed {
        project: ProjectId,
        merge_request: MergeRequestIid,
    },

    MergeRequestMerged {
        project: ProjectId,
        merge_request: MergeRequestIid,
    },

    MergeRequestReopened {
        project: ProjectId,
        merge_request: MergeRequestIid,
    },
}
