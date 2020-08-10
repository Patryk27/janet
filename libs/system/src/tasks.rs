use self::{
    handle_command::handle_command,
    handle_commands::handle_commands,
    handle_event::handle_event,
    handle_events::handle_events,
};

mod handle_command;
mod handle_commands;
mod handle_event;
mod handle_events;

use crate::{CommandRx, EventRx, SystemDeps};
use anyhow::*;
use std::sync::Arc;
use tokio::try_join;

/// Spawns all Janet's background tasks responsible for handling commands &
/// events.
///
/// Returns a `Future` that must be `.await`ed for Janet to work.
pub async fn spawn(ctxt: SystemDeps, cmds: CommandRx, evts: EventRx) -> Result<()> {
    let ctxt = Arc::new(ctxt);

    try_join!(
        handle_commands(ctxt.clone(), cmds),
        handle_events(ctxt, evts),
    )
    .map(drop)
}
