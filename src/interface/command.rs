use crate::gitlab::{DiscussionId, UserId};
use crate::interface::{DateTime, MergeRequestPtr};
use serde::Serialize;
use tokio::sync::mpsc;

mod parse;

pub type CommandTx = mpsc::UnboundedSender<Command>;
pub type CommandRx = mpsc::UnboundedReceiver<Command>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    MergeRequest {
        ctxt: MergeRequestCommandContext,
        cmd: MergeRequestCommand,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CommandAction {
    Add,
    Remove,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum MergeRequestCommand {
    Hi,

    ManageDependency {
        action: CommandAction,
        dependency: MergeRequestPtr,
    },

    ManageReminder {
        action: CommandAction,
        remind_at: DateTime,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MergeRequestCommandContext {
    pub user: UserId,
    pub merge_request: MergeRequestPtr,
    pub discussion: DiscussionId,
}

impl CommandAction {
    pub fn is_add(self) -> bool {
        self == Self::Add
    }

    pub fn is_remove(self) -> bool {
        self == Self::Remove
    }
}
