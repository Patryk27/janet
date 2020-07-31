use crate::interface::MergeRequestPtr;
use serde::Serialize;
use tokio::sync::mpsc;

pub type EventTx = mpsc::UnboundedSender<Event>;
pub type EventRx = mpsc::UnboundedReceiver<Event>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    MergeRequestClosed(MergeRequestPtr),
    MergeRequestMerged(MergeRequestPtr),
    MergeRequestReopened(MergeRequestPtr),
}
