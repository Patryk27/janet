use crate::database::Database;
use crate::gitlab::GitLabClient;
use crate::interface::{CommandRx, EventRx};
use anyhow::Result;
use chrono::Utc;
use std::future::Future;
use std::sync::Arc;
use tokio::select;
use tokio::stream::StreamExt;
use tokio::task;
use tokio::time::{delay_for, Duration};

pub async fn launch(
    db: Database,
    gitlab: Arc<GitLabClient>,
    mut cmds: CommandRx,
    mut evts: EventRx,
) {
    task::spawn(supervise("track_reminders", db.clone(), move |db| {
        track_reminders(db)
    }));

    task::spawn(supervise(
        "track_merge_request_dependencies",
        db.clone(),
        move |db| track_merge_request_dependencies(db),
    ));

    loop {
        let cmd = cmds.next();
        let evt = evts.next();

        select! {
            cmd = cmd => {
                let cmd = cmd.expect("Lost the `commands` stream!");
                log::debug!("Got a command: {:#?}", cmd);
            }

            evt = evt => {
                let evt = evt.expect("Lost the `events` stream");
                log::debug!("Got an event: {:#?}", evt);
            }
        }
    }
}

async fn track_reminders(db: Database) -> Result<()> {
    let reminders = db.reminders();

    loop {
        for reminder in reminders.find_overdue(Utc::now()).await? {
            // TODO
            db.reminders().delete(reminder.id).await?;
        }

        delay_for(Duration::from_secs(5)).await;
    }
}

async fn track_merge_request_dependencies(db: Database) -> Result<()> {
    let merge_request_deps = db.merge_request_dependencies();

    loop {
        for dep in merge_request_deps.find_stale(Utc::now()).await? {
            // TODO
        }
    }
}

async fn supervise<Proc, ProcCtxt>(
    proc_name: &str,
    proc_ctxt: ProcCtxt,
    start_proc: impl Fn(ProcCtxt) -> Proc + Clone,
) where
    Proc: Future<Output = Result<()>>,
    ProcCtxt: Clone,
{
    loop {
        if let Err(err) = start_proc(proc_ctxt.clone()).await {
            log::error!("Process `{}` died: {}", proc_name, err);
            log::error!("... restarting it");

            delay_for(Duration::from_secs(1)).await;
        }
    }
}
