use crate::database::Database;
use crate::gitlab::GitLabClient;
use crate::interface::{Command, CommandTx, Event, EventTx};
use std::sync::Arc;
use tokio::sync::mpsc;

mod processes;

#[derive(Clone, Debug)]
pub struct Cpu {
    cmd_tx: CommandTx,
    evt_tx: EventTx,
}

impl Cpu {
    pub fn init(db: Database, gitlab: Arc<GitLabClient>) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (evt_tx, evt_rx) = mpsc::unbounded_channel();

        processes::launch(db, gitlab, cmd_rx, evt_rx);

        Self { cmd_tx, evt_tx }
    }

    pub fn handle_command(&self, cmd: Command) {
        self.cmd_tx.send(cmd).unwrap();
    }

    pub fn handle_event(&self, evt: Event) {
        self.evt_tx.send(evt).unwrap();
    }
}
