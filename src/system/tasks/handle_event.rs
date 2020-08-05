use self::handle_merge_request::handle_merge_request;

mod handle_merge_request;

use crate::interface::Event;
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;

#[tracing::instrument(skip(ctxt))]
pub async fn handle_event(ctxt: Arc<TaskContext>, evt: Event) {
    tracing::debug!("Handling event");

    match try_handling_event(ctxt, evt).await {
        Ok(_) => {
            tracing::info!("Event handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle event");
        }
    }
}

async fn try_handling_event(ctxt: Arc<TaskContext>, evt: Event) -> Result<()> {
    ctxt.db.logs().add((&evt).into()).await?;

    match evt {
        Event::MergeRequestClosed {
            project,
            merge_request,
        } => {
            handle_merge_request(ctxt, project, merge_request, "closed").await?;
        }

        Event::MergeRequestMerged {
            project,
            merge_request,
        } => {
            handle_merge_request(ctxt, project, merge_request, "merged").await?;
        }

        Event::MergeRequestReopened {
            project,
            merge_request,
        } => {
            handle_merge_request(ctxt, project, merge_request, "reopened").await?;
        }
    }

    Ok(())
}
