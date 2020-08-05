use crate::database::{Id, MergeRequest, MergeRequestDependency};
use crate::gitlab::{DiscussionId, MergeRequestIid, ProjectId, UserId};
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;

/// Handles a generic "state of merge request changed" event.
///
/// `verb` specifies what happened to the merge request (e.g. <<it got>>
/// `closed`) and it's passed verbatim to the message sent to user.
pub async fn handle_merge_request_state_changed(
    ctxt: Arc<TaskContext>,
    project: ProjectId,
    merge_request: MergeRequestIid,
    verb: &'static str,
) -> Result<()> {
    // Since Janet listens for events from all merge requests, it might happen (and
    // will happen) that we'll receive an event about a merge request we don't care
    // about (e.g. for a merge request no one has `+depends on` yet).
    //
    // When that happens, we just want to silently ignore the event - and that's
    // what happens here: we load merge request from our database and when it's not
    // present, we ignore the event.
    let merge_request = if let Some(merge_request) = ctxt
        .db
        .merge_requests()
        .find_by_external_id(project.inner() as _, merge_request.inner() as _)
        .await?
    {
        merge_request
    } else {
        return Ok(());
    };

    notify_merge_request_dependencies(&ctxt, merge_request, verb).await?;

    Ok(())
}

/// Checks dependencies for given merge request and dispatches notes for
/// interested users.
async fn notify_merge_request_dependencies(
    ctxt: &TaskContext,
    merge_request: Id<MergeRequest>,
    verb: &'static str,
) -> Result<()> {
    let deps = ctxt
        .db
        .merge_request_dependencies()
        .find_by_dep(merge_request)
        .await?;

    for dep in deps {
        notify_merge_request_dependency(ctxt, verb, dep).await;
    }

    Ok(())
}

#[tracing::instrument(skip(ctxt))]
async fn notify_merge_request_dependency(
    ctxt: &TaskContext,
    verb: &'static str,
    dep: MergeRequestDependency,
) {
    tracing::trace!("Sending note for merge request dependency");

    if let Err(err) = try_notify_merge_request_dependency(ctxt, verb, dep).await {
        // TODO when someone removes the discussion, this invocation will return 404 -
        //      we should detect it and remove merge request dependency from the
        //      database not to spam the API

        panic!("{:#?}", err);

        // We don't want for this error to get propagated, because - whatever the error
        // says - we want to dispatch as many notes as possible; if we don't send one
        // comment, let's at least rest assured knowing the rest got out
        tracing::error!({ err = ?err }, "Failed to send note");
    }
}

async fn try_notify_merge_request_dependency(
    ctxt: &TaskContext,
    verb: &'static str,
    dep: MergeRequestDependency,
) -> Result<()> {
    let src_merge_request = ctxt
        .db
        .merge_requests()
        .get(dep.src_merge_request_id)
        .await?;

    let dst_merge_request = ctxt
        .db
        .merge_requests()
        .get(dep.dst_merge_request_id)
        .await?;

    let src_project = ctxt.db.projects().get(src_merge_request.project_id).await?;
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
            &DiscussionId::new(dep.discussion_ext_id),
            note,
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::to_json;
    use crate::{database as db, gitlab as gl};

    mod given_untracked_merge_request {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn does_nothing() {
            let ctxt = TaskContext::mock().await;

            handle_merge_request_state_changed(
                ctxt,
                ProjectId::new(123),
                MergeRequestIid::new(4),
                "merged",
            )
            .await
            .unwrap();
        }
    }

    mod given_merge_request_with_tracked_dependencies {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn dispatches_notifications_to_interested_users() {
            let ctxt = TaskContext::mock().await;

            let user_id = ctxt
                .db
                .users()
                .add(&db::NewUser { ext_id: 100 })
                .await
                .unwrap();

            let project_id = ctxt
                .db
                .projects()
                .add(&db::NewProject { ext_id: 123 })
                .await
                .unwrap();

            let src_merge_request_id = ctxt
                .db
                .merge_requests()
                .add(&db::NewMergeRequest {
                    project_id,
                    ext_id: 150,
                    iid: 1,
                    state: "opened".to_string(),
                })
                .await
                .unwrap();

            let dst_merge_request_id = ctxt
                .db
                .merge_requests()
                .add(&db::NewMergeRequest {
                    project_id,
                    ext_id: 151,
                    iid: 2,
                    state: "opened".to_string(),
                })
                .await
                .unwrap();

            ctxt.db
                .merge_request_dependencies()
                .add(&db::NewMergeRequestDependency {
                    user_id,
                    discussion_ext_id: "cafebabe".to_string(),
                    src_merge_request_id,
                    dst_merge_request_id,
                })
                .await
                .unwrap();

            // Invocation #1: this call shouldn't do anything, because it's the _second_
            // merge request that we're waiting for (in other words: merge request #1
            // depends on merge request #2, not the other way around).
            //
            // Asserting this call is a no-op allows us to ensure that we didn't get `src`
            // and `dst` mixed up somewhere in the system.
            {
                handle_merge_request_state_changed(
                    ctxt.clone(),
                    ProjectId::new(123),
                    MergeRequestIid::new(1),
                    "merged",
                )
                .await
                .unwrap();
            }

            // Invocation #2: this call should dispatch one notification to the user #100
            {
                let user_mock = mockito::mock("GET", "/gitlab/api/v4/users/100")
                    .with_body(to_json(&gl::User {
                        id: gl::UserId::new(123),
                        username: "someone".to_string(),
                    }))
                    .create();

                let dst_merge_request_mock =
                    mockito::mock("GET", "/gitlab/api/v4/projects/123/merge_requests/2")
                        .with_body(to_json(&gl::MergeRequest {
                            id: gl::MergeRequestId::new(1024),
                            iid: gl::MergeRequestIid::new(2),
                            project_id: gl::ProjectId::new(123),
                            state: "opened".to_string(),
                            web_url: "http://merge-request".to_string(),
                        }))
                        .create();

                let note_mock = mockito::mock(
                    "POST",
                    "/gitlab/api/v4/projects/123/merge_requests/1/discussions/cafebabe/notes",
                )
                    .match_body(
                        r#"{"body":"@someone related merge request http://merge-request has been merged"}"#,
                    )
                    .create();

                handle_merge_request_state_changed(
                    ctxt,
                    ProjectId::new(123),
                    MergeRequestIid::new(2),
                    "merged",
                )
                .await
                .unwrap();

                user_mock.assert();
                dst_merge_request_mock.assert();
                note_mock.assert();
            }
        }
    }
}
