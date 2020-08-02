use std::sync::Arc;

use anyhow::{bail, Result};
use tokio::stream::StreamExt;

use crate::database::Database;
use crate::gitlab::{DiscussionId, GitLabClient, MergeRequestIid, ProjectId, UserId};
use crate::interface::{Event, EventRx};

pub async fn handle_events(
    db: Database,
    gitlab: Arc<GitLabClient>,
    mut evts: EventRx,
) -> Result<()> {
    while let Some(evt) = evts.next().await {
        let db = db.clone();
        let gitlab = gitlab.clone();

        tokio::spawn(handle_event(db, gitlab, evt));
    }

    bail!("Lost connection to the `events` stream")
}

#[tracing::instrument(skip(db, gitlab))]
async fn handle_event(db: Database, gitlab: Arc<GitLabClient>, evt: Event) {
    tracing::debug!("Handling event");

    match handle_event_inner(db, gitlab, &evt).await {
        Ok(_) => {
            tracing::info!("Event handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle event");
        }
    }
}

async fn handle_event_inner(db: Database, gitlab: Arc<GitLabClient>, evt: &Event) -> Result<()> {
    db.logs().add(evt.into()).await?;

    match evt {
        Event::MergeRequestClosed {
            project_id,
            merge_request_iid,
        }
        | Event::MergeRequestMerged {
            project_id,
            merge_request_iid,
        }
        | Event::MergeRequestReopened {
            project_id,
            merge_request_iid,
        } => {
            let deps = db
                .merge_request_dependencies()
                .find_depending(project_id.inner() as _, merge_request_iid.inner() as _)
                .await?;

            for dep in deps {
                tracing::trace!({ dep = ?dep }, "Sending note");

                let result: Result<()> = try {
                    let user_id = UserId::new(dep.user_id as _);
                    let project_id = ProjectId::new(dep.source_project_id as _);
                    let discussion_id = DiscussionId::new(dep.source_discussion_id.clone());
                    let merge_request_iid = MergeRequestIid::new(dep.source_merge_request_iid as _);

                    let user = gitlab.user(user_id).await?;
                    let merge_request = gitlab.merge_request(project_id, merge_request_iid).await?;

                    let verb = match evt {
                        Event::MergeRequestClosed { .. } => "closed",
                        Event::MergeRequestMerged { .. } => "merged",
                        Event::MergeRequestReopened { .. } => "reopened",

                        // Safety: the topmost `match` already ensures it's one of those events
                        _ => unreachable!(),
                    };

                    let note = format!(
                        "@{} related merge request {} has been {}",
                        user.username, merge_request.web_url, verb,
                    );

                    gitlab
                        .create_merge_request_note(
                            project_id,
                            merge_request_iid,
                            &discussion_id,
                            note,
                        )
                        .await?;
                };

                if let Err(err) = result {
                    tracing::error!({ dep = ?dep, err = ?err }, "Failed to send note");
                }
            }
        }
    }

    Ok(())
}
