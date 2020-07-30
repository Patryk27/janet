use crate::database::Database;
use crate::gitlab::GitLabClient;
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

        db.logs().add(&evt).await.unwrap();

        match evt {
            Event::MergeRequestClosed(merge_request) => {
                // let (project_id, merge_request_iid) = merge_request
                //     .resolve(&gitlab)
                //     .await
                //     .unwrap();
                //
                // let mrs = db
                //     .merge_request_dependencies()
                //     .find_depending(project_id.inner() as _,
                // merge_request_iid.inner() as _)     .await
                //     .unwrap();
                //
                // for mr in mrs {
                //     let project_id = ProjectId::new(mr.source_project_id as
                // _);     let merge_request_iid =
                // MergeRequestIid::new(mr.source_merge_request_iid as _);
                //
                //     gitlab
                //         .create_merge_request_note(project_id,
                // merge_request_iid, "@someone yass!")
                //         .await
                //         .unwrap();
                //
                //     // TODO delete it
                // }
            }

            _ => (),
        }
    }

    bail!("Lost connection to the `events` stream")
}
