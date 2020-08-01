use self::parse::*;
use crate::gitlab::UserId;
use crate::interface::{DateTimeSpec, MergeRequestPtr};
use anyhow::Result;
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
        source: MergeRequestPtr,
        dependency: MergeRequestPtr,
    },

    RemoveMergeRequestDependency {
        user: UserId,
        source: MergeRequestPtr,
        dependency: MergeRequestPtr,
    },

    AddReminder {
        user: UserId,
        merge_request: MergeRequestPtr,
        remind_at: DateTimeSpec,
    },
}

impl Command {
    pub fn parse(user: UserId, merge_request: MergeRequestPtr, cmd: &str) -> Result<Command> {
        log::debug!(
            "parse(); cmd={}, user={}, merge_request={:?}",
            cmd,
            user.inner(),
            merge_request,
        );

        Ok(parse(user, merge_request, cmd).unwrap().1) // TODO
    }
}
