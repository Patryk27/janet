use crate::database::{Database, NewMergeRequestDependency};
use crate::gitlab::GitLabClient;
use crate::interface::{Command, CommandRx, PtrContext};
use anyhow::{bail, Result};
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
        Command::AddMergeRequestDependency {
            user,
            discussion,
            source,
            dependency,
        }
        | Command::RemoveMergeRequestDependency {
            user,
            discussion,
            source,
            dependency,
        } => {
            let user = gitlab.user(user).await?;

            let (source_project_id, source_merge_request_iid) =
                source.resolve(&gitlab, &Default::default()).await?;

            let source_project = gitlab
                .project(source_project_id.inner().to_string())
                .await?;

            let ctxt = PtrContext {
                namespace_id: Some(source_project.namespace.id),
                project_id: Some(source_project.id),
            };

            let (dependency_project_id, dependency_merge_request_iid) =
                dependency.resolve(&gitlab, &ctxt).await?;

            if gitlab
                .merge_request(dependency_project_id, dependency_merge_request_iid)
                .await
                .is_ok()
            {
                db.merge_request_dependencies()
                    .add(&NewMergeRequestDependency {
                        user_id: user.id.inner() as _,
                        source_project_id: source_project_id.inner() as _,
                        source_merge_request_iid: source_merge_request_iid.inner() as _,
                        source_discussion_id: discussion.as_ref().into(),
                        dependency_project_id: dependency_project_id.inner() as _,
                        dependency_merge_request_iid: dependency_merge_request_iid.inner() as _,
                    })
                    .await?;

                gitlab
                    .create_merge_request_note(
                        source_project_id,
                        source_merge_request_iid,
                        &discussion,
                        format!("@{} :+1:", user.username),
                    )
                    .await?;
            } else {
                gitlab
                    .create_merge_request_note(
                        source_project_id,
                        source_merge_request_iid,
                        &discussion,
                        format!("@{} sorry, I couldn't find this merge request - could you please ensure it exists and re-create / delete your comment?", user.username),
                    )
                    .await?;
            }
        }

        _ => (),
    }

    Ok(())
}
