pub use self::{action::*, merge_request::*};

mod action;
mod merge_request;

use serde::Serialize;
use tokio::sync::mpsc;

pub type CommandTx = mpsc::UnboundedSender<Command>;
pub type CommandRx = mpsc::UnboundedReceiver<Command>;

/// A generic command accepted by Janet.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    MergeRequest {
        ctxt: MergeRequestCommandContext,
        cmd: MergeRequestCommand,
    },
}
