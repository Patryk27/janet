mod handle_commands;
mod handle_events;
mod track_reminders;

use crate::prelude::*;
use tokio::try_join;

/// Spawns all Janet's background tasks responsible for handling commands &
/// events.
///
/// Returns a `Future` that must be `.await`ed for Janet to work.
pub async fn spawn(world: World, cmds: CommandRx, evts: EventRx) -> Result<()> {
    let world = Arc::new(world);

    try_join!(
        handle_commands::start(world.clone(), cmds),
        handle_events::start(world.clone(), evts),
        track_reminders::start(world),
    )
    .map(drop)
}
