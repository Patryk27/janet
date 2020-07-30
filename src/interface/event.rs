use crate::gitlab::MergeRequestPtr;
use serde::Serialize;
use tokio::sync::mpsc;

pub type EventTx = mpsc::UnboundedSender<Event>;
pub type EventRx = mpsc::UnboundedReceiver<Event>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Event {
    MergeRequestClosed(MergeRequestPtr),
    MergeRequestMerged(MergeRequestPtr),
}
