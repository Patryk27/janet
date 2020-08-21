mod merge_request;

use crate::{CommandPacket, SystemDeps};
use anyhow::*;
use lib_database::CreateLogEntry;
use lib_interface::Command;
use std::sync::Arc;

#[tracing::instrument(skip(deps, packet))]
pub async fn handle_command(deps: Arc<SystemDeps>, packet: CommandPacket) {
    match try_handle_command(deps, packet.command).await {
        Ok(_) => {
            tracing::info!("Command handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle command");
        }
    }

    if let Some(responder) = packet.responder {
        let _ = responder.send(());
    }
}

#[tracing::instrument(skip(deps))]
pub async fn try_handle_command(deps: Arc<SystemDeps>, cmd: Command) -> Result<()> {
    tracing::debug!("Handling command");

    deps.db
        .execute(CreateLogEntry {
            event: "command".to_string(),
            payload: serde_json::to_string(&cmd).unwrap(),
        })
        .await?;

    match cmd {
        Command::MergeRequest { ctxt, cmd } => merge_request::handle(&deps, ctxt, cmd).await,
    }
}
