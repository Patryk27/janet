use crate::prelude::*;

mod merge_request_state_changed;

/// Starts an eternal loop that watches for incoming events and processes them
pub async fn start(world: Arc<World>, mut events: EventRx) -> Result<()> {
    while let Some(event) = events.next().await {
        tokio::spawn(handle_event(world.clone(), event));
    }

    bail!("Lost connection to the `events` stream")
}

#[tracing::instrument(skip(world))]
async fn handle_event(world: Arc<World>, packet: Packet<int::Event>) {
    match try_handle_event(world, packet.item).await {
        Ok(_) => {
            tracing::info!("Event handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle event");
        }
    }

    let _ = packet.on_handled.send(());
}

#[tracing::instrument(skip(world, event))]
async fn try_handle_event(world: Arc<World>, event: int::Event) -> Result<()> {
    tracing::debug!("Handling event");

    world
        .db
        .execute(db::CreateLogEntry {
            event: "event".to_string(),
            payload: serde_json::to_string(&event)?,
        })
        .await?;

    match event {
        int::Event::MergeRequestClosed {
            project,
            merge_request,
        } => {
            merge_request_state_changed::handle(&world, project, merge_request, "closed").await?;
        }

        int::Event::MergeRequestMerged {
            project,
            merge_request,
        } => {
            merge_request_state_changed::handle(&world, project, merge_request, "merged").await?;
        }

        int::Event::MergeRequestReopened {
            project,
            merge_request,
        } => {
            merge_request_state_changed::handle(&world, project, merge_request, "reopened").await?;
        }
    }

    Ok(())
}
