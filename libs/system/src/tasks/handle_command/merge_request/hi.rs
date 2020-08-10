use super::HandlerResult;
use crate::SystemDeps;
use lib_interface::MergeRequestCommandContext;

/// Handles the `hi` command.
pub async fn handle(deps: &SystemDeps, ctxt: &MergeRequestCommandContext) -> HandlerResult<()> {
    let gl_user = deps.gitlab.user(ctxt.user).await?;

    let (gl_project_id, gl_merge_request_iid) = ctxt
        .merge_request
        .resolve(&deps.gitlab, &Default::default())
        .await?;

    deps.gitlab
        .create_merge_request_note(
            gl_project_id,
            gl_merge_request_iid,
            &ctxt.discussion,
            format!("Hi, @{}!", gl_user.username),
        )
        .await?;

    Ok(())
}

#[cfg(test)]
#[cfg(feature = "e2e")]
mod tests {
    use lib_e2e::*;

    mod when_user_adds_comment {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn responds_with_greeting() {
            test(async move |ctxt| {
                ctxt.gitlab.expect_user(1, &gl_mock::user_250()).await;

                ctxt.gitlab
                    .expect_merge_request_note_created(
                        gl::ProjectId::new(10),
                        gl::MergeRequestIid::new(1),
                        &gl::DiscussionId::new("cafebabe"),
                        "Hi, @someone!",
                    )
                    .await;

                ctxt.janet
                    .spoof_gitlab_webhook(&json!({
                        "event_type": "note",
                        "project": {
                            "id": 10,
                            "namespace": "alpha",
                        },
                        "merge_request": {
                            "id": 100,
                            "iid": 1,
                        },
                        "object_attributes": {
                            "author_id": 250,
                            "description": "@janet hi!!!",
                            "discussion_id": "cafebabe",
                        },
                    }))
                    .await;
            })
            .await;
        }
    }
}
