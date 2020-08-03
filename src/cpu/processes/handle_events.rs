//! TODO proof of concept - requires solid refactoring

use crate::database::Database;
use crate::gitlab::{DiscussionId, GitLabClient, MergeRequestIid, ProjectId, UserId};
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
        let db = db.clone();
        let gitlab = gitlab.clone();

        tokio::spawn(handle_event(db, gitlab, evt));
    }

    bail!("Lost connection to the `events` stream")
}

#[tracing::instrument(skip(db, gitlab))]
async fn handle_event(db: Database, gitlab: Arc<GitLabClient>, evt: Event) {
    tracing::debug!("Handling event");

    match handle_event_inner(db, gitlab, evt).await {
        Ok(_) => {
            tracing::info!("Event handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle event");
        }
    }
}

async fn handle_event_inner(db: Database, gitlab: Arc<GitLabClient>, evt: Event) -> Result<()> {
    db.logs().add((&evt).into()).await?;

    match evt {
        Event::MergeRequestClosed {
            project,
            merge_request,
        }
        | Event::MergeRequestMerged {
            project,
            merge_request,
        }
        | Event::MergeRequestReopened {
            project,
            merge_request,
        } => {
            let merge_request = if let Some(merge_request) = db
                .merge_requests()
                .find_by_ext(project.inner() as _, merge_request.inner() as _)
                .await?
            {
                merge_request
            } else {
                return Ok(());
            };

            let deps = db
                .merge_request_dependencies()
                .find_by_dep(merge_request)
                .await?;

            for dep in deps {
                tracing::trace!({ dep = ?dep }, "Sending note");

                let result: Result<()> = try {
                    let src_merge_request =
                        db.merge_requests().get(dep.src_merge_request_id).await?;

                    let src_project = db.projects().get(src_merge_request.project_id).await?;

                    let dst_merge_request =
                        db.merge_requests().get(dep.dst_merge_request_id).await?;

                    let dst_project = db.projects().get(dst_merge_request.project_id).await?;

                    let user = db.users().get(dep.user_id).await?;

                    let gl_user = gitlab.user(UserId::new(user.ext_id as _)).await?;

                    let gl_dst_merge_request = gitlab
                        .merge_request(
                            ProjectId::new(dst_project.ext_id as _),
                            MergeRequestIid::new(dst_merge_request.iid as _),
                        )
                        .await?;

                    let verb = match evt {
                        Event::MergeRequestClosed { .. } => "closed",
                        Event::MergeRequestMerged { .. } => "merged",
                        Event::MergeRequestReopened { .. } => "reopened",
                    };

                    let note = format!(
                        "@{} related merge request {} has been {}",
                        gl_user.username, gl_dst_merge_request.web_url, verb,
                    );

                    gitlab
                        .create_merge_request_note(
                            ProjectId::new(src_project.ext_id as _),
                            MergeRequestIid::new(src_merge_request.iid as _),
                            &DiscussionId::new(dep.discussion_ext_id.clone()),
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
