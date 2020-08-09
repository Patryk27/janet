use crate::database::Database;
use crate::gitlab::GitLabClient;
use crate::interface::{Command, CommandTx, Event, EventTx};
use anyhow::*;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::mpsc;

pub(self) use self::deps::*;

mod deps;
mod tasks;
mod utils;

#[derive(Clone, Debug)]
pub struct System {
    cmd_tx: CommandTx,
    evt_tx: EventTx,
}

impl System {
    pub fn init(
        db: Database,
        gitlab: Arc<GitLabClient>,
    ) -> (Arc<Self>, impl Future<Output = Result<()>>) {
        let ctxt = SystemDeps { db, gitlab };

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (evt_tx, evt_rx) = mpsc::unbounded_channel();

        let this = Arc::new(Self { cmd_tx, evt_tx });
        let task = tasks::spawn(ctxt, cmd_rx, evt_rx);

        (this, task)
    }

    pub fn send_cmd(&self, cmd: Command) {
        self.cmd_tx.send(cmd).unwrap();
    }

    pub fn send_evt(&self, evt: Event) {
        self.evt_tx.send(evt).unwrap();
    }
}
