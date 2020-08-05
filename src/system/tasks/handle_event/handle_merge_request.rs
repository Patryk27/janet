//! TODO needs refactoring

use crate::gitlab::{DiscussionId, MergeRequestIid, ProjectId, UserId};
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;

pub async fn handle_merge_request(
    ctxt: Arc<TaskContext>,
    project: ProjectId,
    merge_request: MergeRequestIid,
    verb: &'static str,
) -> Result<()> {
    let merge_request = if let Some(merge_request) = ctxt
        .db
        .merge_requests()
        .find_by_ext(project.inner() as _, merge_request.inner() as _)
        .await?
    {
        merge_request
    } else {
        return Ok(());
    };

    let deps = ctxt
        .db
        .merge_request_dependencies()
        .find_by_dep(merge_request)
        .await?;

    for dep in deps {
        tracing::trace!({ dep = ?dep }, "Sending note");

        let result: Result<()> = try {
            let src_merge_request = ctxt
                .db
                .merge_requests()
                .get(dep.src_merge_request_id)
                .await?;

            let src_project = ctxt.db.projects().get(src_merge_request.project_id).await?;

            let dst_merge_request = ctxt
                .db
                .merge_requests()
                .get(dep.dst_merge_request_id)
                .await?;

            let dst_project = ctxt.db.projects().get(dst_merge_request.project_id).await?;

            let user = ctxt.db.users().get(dep.user_id).await?;

            let gl_user = ctxt.gitlab.user(UserId::new(user.ext_id as _)).await?;

            let gl_dst_merge_request = ctxt
                .gitlab
                .merge_request(
                    ProjectId::new(dst_project.ext_id as _),
                    MergeRequestIid::new(dst_merge_request.iid as _),
                )
                .await?;

            let note = format!(
                "@{} related merge request {} has been {}",
                gl_user.username, gl_dst_merge_request.web_url, verb,
            );

            ctxt.gitlab
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

    Ok(())
}
