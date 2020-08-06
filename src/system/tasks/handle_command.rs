use self::handle_merge_request::*; // TODO use star import everywhere

mod handle_merge_request;

use crate::interface::Command;
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;

#[tracing::instrument(skip(ctxt))]
pub async fn handle_command(ctxt: Arc<TaskContext>, cmd: Command) {
    tracing::debug!("Handling command");

    match try_handle_command(ctxt, cmd).await {
        Ok(_) => {
            tracing::info!("Command handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle command");
        }
    }
}

async fn try_handle_command(tctxt: Arc<TaskContext>, cmd: Command) -> Result<()> {
    tctxt.db.logs().add((&cmd).into()).await?;

    match cmd {
        Command::MergeRequest { ctxt, cmd } => handle_merge_request(tctxt, ctxt, cmd).await,
    }
}
