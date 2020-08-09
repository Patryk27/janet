use super::HandlerResult;
use crate::interface::MergeRequestCommandContext;
use crate::system::SystemDeps;

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
mod tests {
    use super::*;
    use crate::gitlab as gl;
    use crate::interface::*;
    use crate::utils::for_tests::*;
    use std::sync::Arc;

    async fn run(
        deps: &Arc<SystemDeps>,
        ctxt: MergeRequestCommandContext,
        cmd: MergeRequestCommand,
    ) {
        let cmd = Command::MergeRequest { ctxt, cmd };

        super::super::super::try_handle_command(Arc::clone(deps), cmd)
            .await
            .unwrap();
    }

    #[tokio::test(threaded_scheduler)]
    async fn responds_hi() {
        let deps = SystemDeps::mock().await;

        let user_mock = mock_default_user();

        let note_mock = mock_note_created(
            gl::ProjectId::new(123),
            gl::MergeRequestIid::new(1),
            &gl::DiscussionId::new("cafebabe"),
            "Hi, @someone!",
        );

        run(
            &deps,
            MergeRequestCommandContext {
                user: gl::UserId::new(100),
                merge_request: MergeRequestPtr::Iid {
                    project: Some(ProjectPtr::Id(gl::ProjectId::new(123))),
                    merge_request: gl::MergeRequestIid::new(1),
                },
                discussion: gl::DiscussionId::new("cafebabe"),
            },
            MergeRequestCommand::Hi,
        )
        .await;

        user_mock.assert();
        note_mock.assert();
    }
}
