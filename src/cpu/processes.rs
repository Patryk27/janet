use self::{
    handle_commands::handle_commands,
    handle_events::handle_events,
    track_merge_request_dependencies::track_merge_request_dependencies,
    track_reminders::track_reminders,
};

use crate::database::Database;
use crate::gitlab::GitLabClient;
use crate::interface::{CommandRx, EventRx};
use std::sync::Arc;
use tokio::{task, try_join};

mod handle_commands;
mod handle_events;
mod track_merge_request_dependencies;
mod track_reminders;

pub fn launch(db: Database, gitlab: Arc<GitLabClient>, mut cmds: CommandRx, mut evts: EventRx) {
    let handle_commands = task::spawn(handle_commands(db.clone(), gitlab.clone(), cmds));

    let handle_events = task::spawn(handle_events(db.clone(), gitlab.clone(), evts));

    let track_merge_request_dependencies =
        task::spawn(track_merge_request_dependencies(db.clone()));

    let track_reminders = task::spawn(track_reminders(db.clone()));

    task::spawn(async move {
        try_join!(
            handle_commands,
            handle_events,
            track_merge_request_dependencies,
            track_reminders
        )
        .unwrap() // todo ayy ayy
    });
}

// TODO re-think the supervising strategy
//
// async fn supervise<Proc, ProcCtxt>(
//     proc_name: &str,
//     proc_ctxt: ProcCtxt,
//     start_proc: impl Fn(ProcCtxt) -> Proc + Clone,
// ) where
//     Proc: Future<Output = Result<()>>,
//     ProcCtxt: Clone,
// {
//     loop {
//         if let Err(err) = start_proc(proc_ctxt.clone()).await {
//             log::error!("Process `{}` died: {}", proc_name, err);
//             log::error!("... restarting it");
//
//             delay_for(Duration::from_secs(1)).await;
//         }
//     }
// }
