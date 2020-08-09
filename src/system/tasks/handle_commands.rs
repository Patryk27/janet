use super::handle_command;
use crate::interface::CommandRx;
use crate::system::SystemDeps;
use anyhow::*;
use std::sync::Arc;
use tokio::stream::StreamExt;
use tokio::task;

pub async fn handle_commands(deps: Arc<SystemDeps>, mut cmds: CommandRx) -> Result<()> {
    while let Some(cmd) = cmds.next().await {
        task::spawn(handle_command(deps.clone(), cmd));
    }

    bail!("Lost connection to the `commands` stream")
}
