use crate::gitlab::{MergeRequestIid, ProjectId};
use serde::Serialize;
use tokio::sync::mpsc;

pub type EventTx = mpsc::UnboundedSender<Event>;
pub type EventRx = mpsc::UnboundedReceiver<Event>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    MergeRequestClosed {
        project_id: ProjectId,
        merge_request_iid: MergeRequestIid,
    },

    MergeRequestMerged {
        project_id: ProjectId,
        merge_request_iid: MergeRequestIid,
    },

    MergeRequestReopened {
        project_id: ProjectId,
        merge_request_iid: MergeRequestIid,
    },
}
