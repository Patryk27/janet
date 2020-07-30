use crate::database::Database;
use crate::gitlab::GitLabClient;
use crate::interface::{Command, CommandRx, PtrContext};
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::stream::StreamExt;

pub async fn handle_commands(
    db: Database,
    gitlab: Arc<GitLabClient>,
    mut cmds: CommandRx,
) -> Result<()> {
    while let Some(cmd) = cmds.next().await {
        log::debug!("Handling command: {:?}", cmd);

        db.logs().add(&cmd).await.unwrap();

        match cmd {
            Command::AddMergeRequestDependency {
                user,
                source,
                dependency,
            } => {
                let (source_project_id, source_merge_request_iid) =
                    source.resolve(&gitlab, &Default::default()).await.unwrap();

                let source_project = gitlab
                    .project(source_project_id.inner().to_string())
                    .await?;

                let ctxt = PtrContext {
                    namespace_id: Some(source_project.namespace.id),
                    project_id: Some(source_project.id),
                    ..Default::default()
                };

                let (dependency_project_id, dependency_merge_request_iid) =
                    dependency.resolve(&gitlab, &ctxt).await.unwrap();

                log::trace!("- source_project_id = {}", source_project_id.inner());

                log::trace!(
                    "- source_merge_request_iid = {}",
                    source_merge_request_iid.inner()
                );

                log::trace!(
                    "- dependency_project_id = {}",
                    dependency_project_id.inner()
                );

                log::trace!(
                    "- dependency_merge_request_id = {}",
                    dependency_merge_request_iid.inner()
                );

                // let dep = NewMergeRequestDependency {
                //     user_id: user.inner() as _,
                //     source_project_id: source_project_id.inner() as _,
                //     source_merge_request_iid:
                // source_merge_request_iid.inner() as _,
                //     dependency_project_id: dependency_project_id.inner() as
                // _,     dependency_merge_request_iid:
                // dependency_merge_request_iid.inner() as _, };
                //
                // let id = db
                //     .merge_request_dependencies()
                //     .add(&dep)
                //     .await
                //     .unwrap();
            }

            _ => (),
        }
    }

    bail!("Lost connection to the `commands` stream")
}
