use crate::database::Database;
use crate::gitlab::{GitLabClient, MergeRequestIid, ProjectId};
use crate::interface::{Event, EventRx};
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::stream::StreamExt;

pub async fn handle_events(
    db: Database,
    gitlab: Arc<GitLabClient>,
    mut evts: EventRx,
) -> Result<()> {
    while let Some(evt) = evts.next().await {
        log::debug!("Handling event: {:?}", evt);

        let db = db.clone();
        let gitlab = gitlab.clone();

        tokio::spawn(async move {
            match handle_event(db, gitlab, &evt).await {
                Ok(_) => {
                    log::debug!("Handled event: {:?}", evt);
                }

                Err(err) => {
                    log::error!("Couldn't handle event {:?}: {:?}", evt, err);
                }
            }
        });
    }

    bail!("Lost connection to the `events` stream")
}

async fn handle_event(db: Database, gitlab: Arc<GitLabClient>, evt: &Event) -> Result<()> {
    db.logs().add(evt).await?;

    match evt {
        Event::MergeRequestClosed(merge_request)
        | Event::MergeRequestMerged(merge_request)
        | Event::MergeRequestReopened(merge_request) => {
            let (project_id, merge_request_iid) =
                merge_request.resolve(&gitlab, &Default::default()).await?;

            let merge_requests = db
                .merge_request_dependencies()
                .find_depending(project_id.inner() as _, merge_request_iid.inner() as _)
                .await?;

            for merge_request in merge_requests {
                let project_id = ProjectId::new(merge_request.source_project_id as _);

                let merge_request_iid =
                    MergeRequestIid::new(merge_request.source_merge_request_iid as _);

                gitlab
                    .create_merge_request_note(
                        project_id.inner().to_string(),
                        merge_request_iid.inner().to_string(),
                        "@someone yass!",
                    )
                    .await?;

                // TODO delete it
            }
        }

        _ => (),
    }

    Ok(())
}
