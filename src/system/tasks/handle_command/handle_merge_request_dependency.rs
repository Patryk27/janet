//! TODO needs refactoring

use crate::database::NewMergeRequestDependency;
use crate::gitlab::{DiscussionId, UserId};
use crate::interface::{CommandAction, MergeRequestPtr, PtrContext};
use crate::system::task::TaskContext;
use crate::system::utils::{sync_merge_request, sync_user};
use anyhow::*;
use std::sync::Arc;

pub async fn handle_merge_request_dependency(
    ctxt: Arc<TaskContext>,
    action: CommandAction,
    user: UserId,
    discussion: DiscussionId,
    source: MergeRequestPtr,
    dependency: MergeRequestPtr,
) -> Result<()> {
    let (gl_user, user_id) = sync_user(&ctxt.db, &ctxt.gitlab, user).await?;

    let (gl_src_project, gl_src_mr, src_mr_id) =
        sync_merge_request(&ctxt.db, &ctxt.gitlab, &source, &Default::default()).await?;

    // TODO when anything below this line fails, we should notify OP

    let (gl_dst_project, gl_dst_mr, dst_mr_id) = sync_merge_request(
        &ctxt.db,
        &ctxt.gitlab,
        &dependency,
        &PtrContext {
            namespace_id: Some(gl_src_project.namespace.id),
            project_id: Some(gl_src_project.id),
        },
    )
    .await?;

    let dependency = ctxt
        .db
        .merge_request_dependencies()
        .find_by_src(user_id, discussion.as_ref(), src_mr_id)
        .await?;

    if action.is_add() {
        if ctxt
            .gitlab
            .merge_request(gl_dst_project.id, gl_dst_mr.iid)
            .await
            .is_ok()
        {
            if dependency.is_none() {
                ctxt.db
                    .merge_request_dependencies()
                    .add(&NewMergeRequestDependency {
                        user_id,
                        discussion_ext_id: discussion.as_ref().into(),
                        src_merge_request_id: src_mr_id,
                        dst_merge_request_id: dst_mr_id,
                    })
                    .await?;
            }

            ctxt.gitlab
                .create_merge_request_note(
                    gl_src_project.id,
                    gl_src_mr.iid,
                    &discussion,
                    format!("@{} :+1:", gl_user.username),
                )
                .await?;
        } else {
            ctxt.gitlab
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
            ctxt.db.merge_request_dependencies().remove(dep.id).await?;
        }

        ctxt.gitlab
            .create_merge_request_note(
                gl_src_project.id,
                gl_src_mr.iid,
                &discussion,
                format!("@{} :+1:", gl_user.username),
            )
            .await?;
    }

    Ok(())
}
