mod merge_request_state_changed;

use crate::{EventPacket, SystemDeps};
use anyhow::*;
use lib_database::CreateLogEntry;
use lib_interface::Event;
use std::sync::Arc;

#[tracing::instrument(skip(deps, packet))]
pub async fn handle_event(deps: Arc<SystemDeps>, packet: EventPacket) {
    match try_handle_event(deps, packet.command).await {
        Ok(_) => {
            tracing::info!("Event handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle event");
        }
    }

    if let Some(responder) = packet.responder {
        let _ = responder.send(());
    }
}

#[tracing::instrument(skip(deps))]
async fn try_handle_event(deps: Arc<SystemDeps>, evt: Event) -> Result<()> {
    tracing::debug!("Handling event");

    deps.db
        .execute(CreateLogEntry {
            event: "event".to_string(),
            payload: serde_json::to_string(&evt).unwrap(),
        })
        .await?;

    match evt {
        Event::MergeRequestClosed {
            project,
            merge_request,
        } => {
            merge_request_state_changed::handle(deps, project, merge_request, "closed").await?;
        }

        Event::MergeRequestMerged {
            project,
            merge_request,
        } => {
            merge_request_state_changed::handle(deps, project, merge_request, "merged").await?;
        }

        Event::MergeRequestReopened {
            project,
            merge_request,
        } => {
            merge_request_state_changed::handle(deps, project, merge_request, "reopened").await?;
        }
    }

    Ok(())
}
