use crate::gitlab::{MergeRequestIid, ProjectId};
use serde::Serialize;
use tokio::sync::mpsc;

pub type EventTx = mpsc::UnboundedSender<Event>;
pub type EventRx = mpsc::UnboundedReceiver<Event>;

/// A generic event accepted by Janet.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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
