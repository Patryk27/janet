use crate::SystemDeps;
use anyhow::*;
use lib_database::{Id, MergeRequest, MergeRequestDependency};
use lib_gitlab::{DiscussionId, MergeRequestIid, ProjectId, UserId};
use std::sync::Arc;

/// Handles a generic "state of merge request changed" event.
///
/// `verb` specifies what happened to the merge request (e.g. <<it got>>
/// `closed`) and it's passed verbatim to the message sent to user.
pub async fn handle(
    deps: Arc<SystemDeps>,
    project: ProjectId,
    merge_request: MergeRequestIid,
    verb: &'static str,
) -> Result<()> {
    // Since Janet listens for events from all merge requests, it might happen (and
    // will happen) that we'll receive an event about a merge request we don't care
    // about (e.g. for a merge request no one has `depends on` yet).
    //
    // When that happens, we just want to silently ignore the event - and that's
    // what happens here: we load merge request from our database and when it's not
    // present, we ignore the event.
    let merge_request = if let Some(merge_request) = deps
        .db
        .merge_requests()
        .find_by_external_id(project.inner() as _, merge_request.inner() as _)
        .await?
    {
        merge_request
    } else {
        return Ok(());
    };

    notify_merge_request_dependencies(&deps, merge_request, verb).await?;

    Ok(())
}

/// Checks dependencies for given merge request and dispatches notes for
/// interested users.
async fn notify_merge_request_dependencies(
    deps: &SystemDeps,
    merge_request: Id<MergeRequest>,
    verb: &'static str,
) -> Result<()> {
    let mr_deps = deps
        .db
        .merge_request_dependencies()
        .find_by_dep(merge_request)
        .await?;

    for mr_dep in mr_deps {
        notify_merge_request_dependency(deps, verb, mr_dep).await;
    }

    Ok(())
}

#[tracing::instrument(skip(deps))]
async fn notify_merge_request_dependency(
    deps: &SystemDeps,
    verb: &'static str,
    mr_dep: MergeRequestDependency,
) {
    tracing::trace!("Sending note for merge request dependency");

    if let Err(err) = try_notify_merge_request_dependency(deps, verb, mr_dep).await {
        // TODO when someone removes the discussion, this invocation will return 404 -
        //      we should detect it and remove merge request dependency from the
        //      database not to spam the API

        // We don't want for this error to get propagated, because - whatever the error
        // says - we want to dispatch as many notes as possible; if we don't send one
        // comment, let's at least rest assured knowing the rest got out
        tracing::error!({ err = ?err }, "Failed to send note");
    }
}

async fn try_notify_merge_request_dependency(
    deps: &SystemDeps,
    verb: &'static str,
    mr_dep: MergeRequestDependency,
) -> Result<()> {
    let src_merge_request = deps
        .db
        .merge_requests()
        .get(mr_dep.src_merge_request_id)
        .await?;

    let dst_merge_request = deps
        .db
        .merge_requests()
        .get(mr_dep.dst_merge_request_id)
        .await?;

    let src_project = deps.db.projects().get(src_merge_request.project_id).await?;
    let dst_project = deps.db.projects().get(dst_merge_request.project_id).await?;
    let user = deps.db.users().get(mr_dep.user_id).await?;

    let gl_user = deps.gitlab.user(UserId::new(user.ext_id as _)).await?;

    let gl_dst_merge_request = deps
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

    deps.gitlab
        .create_merge_request_note(
            ProjectId::new(src_project.ext_id as _),
            MergeRequestIid::new(src_merge_request.iid as _),
            &DiscussionId::new(mr_dep.discussion_ext_id),
            note,
        )
        .await?;

    Ok(())
}
