use crate::prelude::*;

mod merge_request;

/// Starts an eternal loop that watches for incoming commands and processes
/// them
pub async fn start(world: Arc<World>, mut commands: CommandRx) -> Result<()> {
    while let Some(command) = commands.next().await {
        task::spawn(handle_command(world.clone(), command));
    }

    bail!("Lost connection to the `commands` stream")
}

#[tracing::instrument(skip(world))]
async fn handle_command(world: Arc<World>, packet: Packet<int::Command>) {
    match try_handle_command(world, packet.item).await {
        Ok(_) => {
            tracing::info!("Command handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle command");
        }
    }

    let _ = packet.on_handled.send(());
}

#[tracing::instrument(skip(world, cmd))]
async fn try_handle_command(world: Arc<World>, cmd: int::Command) -> Result<()> {
    tracing::debug!("Handling command");

    world
        .db
        .execute(db::CreateLogEntry {
            event: "command".to_string(),
            payload: serde_json::to_string(&cmd)?,
        })
        .await?;

    match cmd {
        int::Command::MergeRequest { ctxt, cmd } => merge_request::handle(&world, ctxt, cmd).await,
    }
}
