use crate::gitlab::{DiscussionId, UserId};
use crate::interface::{DateTimeSpec, MergeRequestPtr};
use serde::Serialize;
use tokio::sync::mpsc;

mod parse;

pub type CommandTx = mpsc::UnboundedSender<Command>;
pub type CommandRx = mpsc::UnboundedReceiver<Command>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    AddMergeRequestDependency {
        user: UserId,
        discussion: DiscussionId,
        source: MergeRequestPtr,
        dependency: MergeRequestPtr,
    },

    RemoveMergeRequestDependency {
        user: UserId,
        discussion: DiscussionId,
        source: MergeRequestPtr,
        dependency: MergeRequestPtr,
    },

    AddReminder {
        user: UserId,
        discussion: DiscussionId,
        merge_request: MergeRequestPtr,
        remind_at: DateTimeSpec,
    },
}
