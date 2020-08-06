//! TODO everything in here feels hacky and overly complicated

use crate::interface::{CommandAction, MergeRequestPtr, PtrContext};
use crate::system::task::TaskContext;
use crate::system::utils::{sync_merge_request, sync_merge_request_ptr, sync_user};
use crate::{database as db, gitlab as gl};
use anyhow::*;
use std::sync::Arc;

enum CmdError {
    MergeRequestNotFound,
    Other(anyhow::Error),
}

struct CmdPayload<'a> {
    user_id: db::Id<db::User>,
    gl_discussion_id: &'a gl::DiscussionId,
    gl_src_project: &'a gl::Project,
    src_merge_request_id: db::Id<db::MergeRequest>,
    dependency: MergeRequestPtr,
}

/// Handles the `+depends on` & `-depends on` commands.
pub async fn handle_manage_dependency(
    ctxt: Arc<TaskContext>,
    action: CommandAction,
    user: gl::UserId,
    discussion: gl::DiscussionId,
    source: MergeRequestPtr,
    dependency: MergeRequestPtr,
) -> Result<()> {
    let (gl_user, user_id) = sync_user(&ctxt.db, &ctxt.gitlab, user).await?;

    let (gl_src_project, gl_src_merge_request, src_merge_request_id) =
        sync_merge_request_ptr(&ctxt.db, &ctxt.gitlab, &source, &Default::default()).await?;

    let payload = CmdPayload {
        user_id,
        gl_discussion_id: &discussion,
        gl_src_project: &gl_src_project,
        src_merge_request_id,
        dependency,
    };

    let response = match try_handle_merge_request_dependency(&ctxt, action, payload).await {
        Ok(response) => {
            response
        },

        Err(CmdError::MergeRequestNotFound) => {
            "sorry, I couldn't find this merge request - could you please ensure it exists and re-create your comment?".into()
        },

        Err(CmdError::Other(error)) => format!(
            "well, this is embarrassing - there was some issue processing your request: {}\
            ; could you please contact the administrator?",
            error,
        ),
    };

    ctxt.gitlab
        .create_merge_request_note(
            gl_src_project.id,
            gl_src_merge_request.iid,
            &discussion,
            format!("@{} {}", gl_user.username, response),
        )
        .await?;

    Ok(())
}

async fn try_handle_merge_request_dependency(
    ctxt: &TaskContext,
    action: CommandAction,
    payload: CmdPayload<'_>,
) -> Result<String, CmdError> {
    let (gl_dst_project_id, gl_dst_merge_request_iid) = payload
        .dependency
        .resolve(
            &ctxt.gitlab,
            &PtrContext {
                namespace_id: Some(payload.gl_src_project.namespace.id),
                project_id: Some(payload.gl_src_project.id),
            },
        )
        .await
        .map_err(|_| CmdError::MergeRequestNotFound)?;

    if ctxt
        .gitlab
        .merge_request(gl_dst_project_id, gl_dst_merge_request_iid)
        .await
        .is_err()
    {
        return Err(CmdError::MergeRequestNotFound);
    }

    let dependency = ctxt
        .db
        .merge_request_dependencies()
        .find_by_src(
            payload.user_id,
            payload.gl_discussion_id.as_ref(),
            payload.src_merge_request_id,
        )
        .await
        .map_err(CmdError::Other)?;

    let (_, _, dst_merge_request_id) = sync_merge_request(
        &ctxt.db,
        &ctxt.gitlab,
        gl_dst_project_id,
        gl_dst_merge_request_iid,
    )
    .await
    .map_err(CmdError::Other)?;

    if action.is_add() {
        try_add_merge_request_dependency(&ctxt, payload, dependency, dst_merge_request_id).await
    } else {
        try_remove_merge_request_dependency(&ctxt, dependency).await
    }
    .map_err(CmdError::Other)
}

/// Handles the `+depends on` command.
async fn try_add_merge_request_dependency(
    ctxt: &TaskContext,
    payload: CmdPayload<'_>,
    dependency: Option<db::MergeRequestDependency>,
    dst_merge_request_id: db::Id<db::MergeRequest>,
) -> Result<String> {
    // It might happen that we already know about this dependency - say, when
    // someone adds the same `+depends on !123` comment twice.
    //
    // In order to make the UI less confusing, when that happens, we're just
    // silently ignoring the second request.
    if dependency.is_none() {
        ctxt.db
            .merge_request_dependencies()
            .add(&db::NewMergeRequestDependency {
                user_id: payload.user_id,
                discussion_ext_id: payload.gl_discussion_id.as_ref().into(),
                src_merge_request_id: payload.src_merge_request_id,
                dst_merge_request_id,
            })
            .await?;
    }

    Ok(":+1:".into())
}

/// Handles the `-depends on` command.
async fn try_remove_merge_request_dependency(
    ctxt: &TaskContext,
    dependency: Option<db::MergeRequestDependency>,
) -> Result<String> {
    // It might happen that we've already removed this dependency - say, when
    // someone adds the same `-depends on !123` comment twice.
    //
    // In order to make the UI less confusing, when that happens, we're just
    // silently ignoring the second request.
    if let Some(dependency) = dependency {
        ctxt.db
            .merge_request_dependencies()
            .remove(dependency.id)
            .await?;
    }

    Ok(":+1:".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::ProjectPtr;
    use crate::utils::for_tests::*;

    mod when_user_tries_to_depend_on_non_existing_merge_request {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn responds_with_error() {
            let ctxt = TaskContext::mock().await;

            let user_mock = mock_default_user();

            let project_mock = mock_project(&gl::Project {
                id: gl::ProjectId::new(123),

                namespace: gl::Namespace {
                    id: gl::NamespaceId::new(1),
                    name: gl::NamespaceName::new("namespace"),
                    full_path: "namespace".to_string(),
                },
            });

            let merge_request_mock = mock_merge_request(&gl::MergeRequest {
                id: gl::MergeRequestId::new(1),
                iid: gl::MergeRequestIid::new(1),
                project_id: gl::ProjectId::new(123),
                state: "opened".to_string(),
                web_url: "http://merge-request".to_string(),
            });

            let note_mock = mock_note_created(
                gl::ProjectId::new(123),
                gl::MergeRequestIid::new(1),
                &gl::DiscussionId::new("cafebabe"),
                "@someone sorry, I couldn't find this merge request - could you please ensure it exists and re-create your comment?"
            );

            handle_manage_dependency(
                ctxt,
                CommandAction::Add,
                gl::UserId::new(100),
                gl::DiscussionId::new("cafebabe"),
                MergeRequestPtr::Iid {
                    project: Some(ProjectPtr::Id(gl::ProjectId::new(123))),
                    merge_request: gl::MergeRequestIid::new(1),
                },
                MergeRequestPtr::Iid {
                    project: Some(ProjectPtr::Id(gl::ProjectId::new(123))),
                    merge_request: gl::MergeRequestIid::new(2),
                },
            )
            .await
            .unwrap();

            user_mock.assert();
        }
    }
}
