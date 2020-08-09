use self::merge_request_state_changed::handle;

mod merge_request_state_changed;

use crate::interface::Event;
use crate::system::SystemDeps;
use anyhow::*;
use std::sync::Arc;

#[tracing::instrument(skip(deps))]
pub async fn handle_event(deps: Arc<SystemDeps>, evt: Event) {
    tracing::debug!("Handling event");

    match try_handle_event(deps, evt).await {
        Ok(_) => {
            tracing::info!("Event handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle event");
        }
    }
}

async fn try_handle_event(deps: Arc<SystemDeps>, evt: Event) -> Result<()> {
    deps.db.logs().add((&evt).into()).await?;

    match evt {
        Event::MergeRequestClosed {
            project,
            merge_request,
        } => {
            handle(deps, project, merge_request, "closed").await?;
        }

        Event::MergeRequestMerged {
            project,
            merge_request,
        } => {
            handle(deps, project, merge_request, "merged").await?;
        }

        Event::MergeRequestReopened {
            project,
            merge_request,
        } => {
            handle(deps, project, merge_request, "reopened").await?;
        }
    }

    Ok(())
}
