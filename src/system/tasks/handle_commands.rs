use super::handle_command;
use crate::interface::CommandRx;
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;
use tokio::stream::StreamExt;
use tokio::task;

pub async fn handle_commands(ctxt: Arc<TaskContext>, mut cmds: CommandRx) -> Result<()> {
    while let Some(cmd) = cmds.next().await {
        task::spawn(handle_command(ctxt.clone(), cmd));
    }

    bail!("Lost connection to the `commands` stream")
}
