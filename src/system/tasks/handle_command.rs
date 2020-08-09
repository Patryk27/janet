mod merge_request;

use crate::interface::Command;
use crate::system::SystemDeps;
use anyhow::*;
use std::sync::Arc;

#[tracing::instrument(skip(deps))]
pub async fn handle_command(deps: Arc<SystemDeps>, cmd: Command) {
    tracing::debug!("Handling command");

    match try_handle_command(deps, cmd).await {
        Ok(_) => {
            tracing::info!("Command handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle command");
        }
    }
}

pub async fn try_handle_command(deps: Arc<SystemDeps>, cmd: Command) -> Result<()> {
    deps.db.logs().add((&cmd).into()).await?;

    match cmd {
        Command::MergeRequest { ctxt, cmd } => merge_request::handle(&deps, ctxt, cmd).await,
    }
}
