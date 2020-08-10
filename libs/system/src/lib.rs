#![feature(async_closure)]

pub(self) use self::{deps::*, packets::*};

use anyhow::*;
use lib_database::Database;
use lib_gitlab::GitLabClient;
use lib_interface::{Command, Event};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::mpsc;

mod deps;
mod packets;
mod tasks;
mod utils;

#[derive(Clone, Debug)]
pub struct System {
    /// When enabled, all the `process_` methods will wait for the command /
    /// event to complete.
    ///
    /// Corresponds to the `--sync` switch.
    sync: bool,

    /// Transmitter allowing to send commands to the system.
    cmd_tx: CommandTx,

    /// Transmitter allowing to send events to the system.
    evt_tx: EventTx,
}

impl System {
    pub fn init(
        sync: bool,
        db: Database,
        gitlab: Arc<GitLabClient>,
    ) -> (Arc<Self>, impl Future<Output = Result<()>>) {
        let deps = SystemDeps { db, gitlab };

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (evt_tx, evt_rx) = mpsc::unbounded_channel();

        let this = Arc::new(Self {
            cmd_tx,
            evt_tx,
            sync,
        });
        let task = tasks::spawn(deps, cmd_rx, evt_rx);

        (this, task)
    }

    /// Sends a command to the system.
    ///
    /// When `sync` is enabled, waits for the command to complete processing;
    /// otherwise returns immediately.
    pub async fn process_command(&self, cmd: Command) {
        if self.sync {
            let (tx, mut rx) = mpsc::unbounded_channel();
            let packet = CommandPacket {
                command: cmd,
                responder: Some(tx),
            };

            self.cmd_tx
                .send(packet)
                .expect("Lost connection with the system");

            rx.recv().await.expect("Lost connection with the system");
        } else {
            let packet = CommandPacket {
                command: cmd,
                responder: None,
            };

            self.cmd_tx
                .send(packet)
                .expect("Lost connection with the system");
        }
    }

    /// Sends an event to the system.
    ///
    /// When `sync` is enabled, waits for the event to complete processing;
    /// otherwise returns immediately.
    pub async fn process_event(&self, evt: Event) {
        if self.sync {
            let (tx, mut rx) = mpsc::unbounded_channel();
            let packet = EventPacket {
                command: evt,
                responder: Some(tx),
            };

            self.evt_tx
                .send(packet)
                .expect("Lost connection with the system");

            rx.recv().await.expect("Lost connection with the system");
        } else {
            let packet = EventPacket {
                command: evt,
                responder: None,
            };

            self.evt_tx
                .send(packet)
                .expect("Lost connection with the system");
        }
    }
}
