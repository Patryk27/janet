use crate::SystemDeps;
use anyhow::*;
use lib_database as db;
use lib_gitlab as gl;
use std::sync::Arc;

/// Handles a generic "state of merge request changed" event.
///
/// `verb` specifies what happened to the merge request (e.g. <<it got>>
/// `closed`) and it's passed verbatim to the message sent to user.
pub async fn handle(
    deps: Arc<SystemDeps>,
    project: gl::ProjectId,
    merge_request: gl::MergeRequestIid,
    verb: &'static str,
) -> Result<()> {
    // Since Janet listens for events from all merge requests, it might happen (and
    // will happen) that we'll receive an event about a merge request we don't care
    // about (e.g. for a merge request no one had `depends on` yet).
    //
    // When that happens, we just want to silently ignore the event - and that's
    // what happens here: we load merge request from our database and when it's not
    // present, we ignore the event.
    let merge_request = deps
        .db
        .maybe_find_one(db::FindMergeRequests {
            ext_iid: Some(merge_request),
            ext_project_id: Some(project),
            ..Default::default()
        })
        .await?;

    let merge_request = if let Some(merge_request) = merge_request {
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
    merge_request: db::MergeRequest,
    verb: &'static str,
) -> Result<()> {
    let mr_deps = deps
        .db
        .find_all(db::FindMergeRequestDependencies {
            dst_merge_request_id: Some(merge_request.id),
            ..Default::default()
        })
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
    mr_dep: db::MergeRequestDependency,
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
    mr_dep: db::MergeRequestDependency,
) -> Result<()> {
    let (src_merge_request, src_project) = {
        let merge_request = deps
            .db
            .find_one(db::FindMergeRequests::id(mr_dep.src_merge_request_id))
            .await?;

        let project = deps
            .db
            .find_one(db::FindProjects::id(merge_request.project_id))
            .await?;

        (merge_request, project)
    };

    let (dst_merge_request, dst_project) = {
        let merge_request = deps
            .db
            .find_one(db::FindMergeRequests::id(mr_dep.dst_merge_request_id))
            .await?;

        let project = deps
            .db
            .find_one(db::FindProjects::id(merge_request.project_id))
            .await?;

        (merge_request, project)
    };

    let user = deps.db.find_one(db::FindUsers::id(mr_dep.user_id)).await?;

    let gl_user = deps.gitlab.user(user.ext_id()).await?;

    let gl_dst_merge_request = deps
        .gitlab
        .merge_request(dst_project.ext_id(), dst_merge_request.ext_iid())
        .await?;

    let note = format!(
        "@{} related merge request {} has been {}",
        gl_user.username, gl_dst_merge_request.web_url, verb,
    );

    deps.gitlab
        .create_merge_request_note(
            src_project.ext_id(),
            src_merge_request.ext_iid(),
            &mr_dep.ext_discussion_id(),
            note,
        )
        .await?;

    Ok(())
}
