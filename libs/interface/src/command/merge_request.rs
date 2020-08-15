mod parse;

use crate::{Command, CommandAction, DateTime, InterfaceError, InterfaceResult, MergeRequestPtr};
use lib_gitlab::{DiscussionId, UserId};
use serde::Serialize;

/// A command issued from the context of a merge request
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum MergeRequestCommand {
    /// E.g.:
    ///
    /// - `hi`
    /// - `hi!!!`
    Hi,

    /// E.g.:
    ///
    /// - `depends on foo!123`
    /// - `-depends on !45`
    ManageDependency {
        action: CommandAction,
        dependency: MergeRequestPtr,
    },

    /// E.g.:
    ///
    /// - `remind me tomorrow`
    /// - `remind in 3d: rebase it!`
    ManageReminder {
        message: Option<String>,
        remind_at: DateTime,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MergeRequestCommandContext {
    /// User who issued the command
    pub user: UserId,

    /// Merge request where the command was issued
    pub merge_request: MergeRequestPtr,

    /// Discussion where the command was issued
    pub discussion: DiscussionId,
}

impl MergeRequestCommand {
    #[tracing::instrument]
    pub fn parse(ctxt: MergeRequestCommandContext, cmd: &str) -> InterfaceResult<Command> {
        tracing::debug!("Parsing command");

        parse::parse(cmd)
            .map(|(_, cmd)| Command::MergeRequest { ctxt, cmd })
            .map_err(|_| InterfaceError::UnknownCommand)
    }
}
