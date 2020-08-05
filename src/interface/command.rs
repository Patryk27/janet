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
    Hi {
        user: UserId,
        discussion: DiscussionId,
        merge_request: MergeRequestPtr,
    },

    MergeRequestDependency {
        action: CommandAction,
        user: UserId,
        discussion: DiscussionId,
        source: MergeRequestPtr,
        dependency: MergeRequestPtr,
    },

    Reminder {
        action: CommandAction,
        user: UserId,
        merge_request: MergeRequestPtr,
        discussion: DiscussionId,
        remind_at: DateTimeSpec,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CommandAction {
    Add,
    Remove,
}

impl CommandAction {
    pub fn is_add(self) -> bool {
        self == Self::Add
    }

    pub fn is_remove(self) -> bool {
        self == Self::Remove
    }
}
