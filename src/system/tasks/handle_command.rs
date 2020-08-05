use self::{
    handle_hi::handle_hi,
    handle_merge_request_dependency::handle_merge_request_dependency,
};

mod handle_hi;
mod handle_merge_request_dependency;

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

async fn try_handle_command(ctxt: Arc<TaskContext>, cmd: Command) -> Result<()> {
    ctxt.db.logs().add((&cmd).into()).await?;

    match cmd {
        Command::Hi {
            user,
            discussion,
            merge_request,
        } => {
            handle_hi(ctxt, user, discussion, merge_request).await?;
        }

        Command::MergeRequestDependency {
            action,
            user,
            discussion,
            source,
            dependency,
        } => {
            handle_merge_request_dependency(ctxt, action, user, discussion, source, dependency)
                .await?;
        }

        _ => (),
    }

    Ok(())
}
