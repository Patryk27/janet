//! TODO proof of concept - requires solid refactoring

use super::super::utils::*;
use crate::database::{Database, NewMergeRequestDependency};
use crate::gitlab::GitLabClient;
use crate::interface::{Command, CommandRx, PtrContext};
use anyhow::*;
use std::sync::Arc;
use tokio::stream::StreamExt;
use tokio::task;

pub async fn handle_commands(
    db: Database,
    gitlab: Arc<GitLabClient>,
    mut cmds: CommandRx,
) -> Result<()> {
    while let Some(cmd) = cmds.next().await {
        let db = db.clone();
        let gitlab = gitlab.clone();

        task::spawn(handle_command(db, gitlab, cmd));
    }

    bail!("Lost connection to the `commands` stream")
}

#[tracing::instrument(skip(db, gitlab))]
async fn handle_command(db: Database, gitlab: Arc<GitLabClient>, cmd: Command) {
    tracing::debug!("Handling command");

    match handle_command_inner(db, gitlab, cmd).await {
        Ok(_) => {
            tracing::info!("Command handled");
        }

        Err(err) => {
            tracing::error!({ err = ?err }, "Failed to handle command");
        }
    }
}

async fn handle_command_inner(db: Database, gitlab: Arc<GitLabClient>, cmd: Command) -> Result<()> {
    db.logs().add((&cmd).into()).await?;

    match cmd {
        Command::MergeRequestDependency {
            action,
            user,
            discussion,
            source,
            dependency,
        } => {
            let (gl_user, user_id) = sync_user(&db, &gitlab, user).await?;

            let (gl_src_project, gl_src_mr, src_mr_id) =
                sync_merge_request(&db, &gitlab, &source, &Default::default()).await?;

            let (gl_dst_project, gl_dst_mr, dst_mr_id) = sync_merge_request(
                &db,
                &gitlab,
                &dependency,
                &PtrContext {
                    namespace_id: Some(gl_src_project.namespace.id),
                    project_id: Some(gl_src_project.id),
                },
            )
            .await?;

            let dependency = db
                .merge_request_dependencies()
                .find_by_src(user_id, discussion.as_ref(), src_mr_id)
                .await?;

            if action.is_add() {
                if gitlab
                    .merge_request(gl_dst_project.id, gl_dst_mr.iid)
                    .await
                    .is_ok()
                {
                    if dependency.is_none() {
                        db.merge_request_dependencies()
                            .add(&NewMergeRequestDependency {
                                user_id,
                                discussion_ext_id: discussion.as_ref().into(),
                                src_merge_request_id: src_mr_id,
                                dst_merge_request_id: dst_mr_id,
                            })
                            .await?;
                    }

                    gitlab
                        .create_merge_request_note(
                            gl_src_project.id,
                            gl_src_mr.iid,
                            &discussion,
                            format!("@{} :+1:", gl_user.username),
                        )
                        .await?;
                } else {
                    gitlab
                        .create_merge_request_note(
                            gl_src_project.id,
                            gl_src_mr.iid,
                            &discussion,
                            format!(
                                "@{} sorry, I couldn't find this merge request - could you please ensure it exists and re-create / delete your comment?",
                                gl_user.username
                            ),
                        )
                        .await?;
                }
            } else {
                if let Some(dep) = dependency {
                    db.merge_request_dependencies().remove(dep.id).await?;
                }

                gitlab
                    .create_merge_request_note(
                        gl_src_project.id,
                        gl_src_mr.iid,
                        &discussion,
                        format!("@{} :+1:", gl_user.username),
                    )
                    .await?;
            }
        }

        _ => (),
    }

    Ok(())
}
