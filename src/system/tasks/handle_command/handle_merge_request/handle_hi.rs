use crate::gitlab::{DiscussionId, UserId};
use crate::interface::MergeRequestPtr;
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;

pub async fn handle_hi(
    ctxt: Arc<TaskContext>,
    user: UserId,
    discussion: DiscussionId,
    merge_request: MergeRequestPtr,
) -> Result<()> {
    let user = ctxt.gitlab.user(user).await?;

    let (project_id, merge_request_iid) = merge_request
        .resolve(&ctxt.gitlab, &Default::default())
        .await?;

    ctxt.gitlab
        .create_merge_request_note(
            project_id,
            merge_request_iid,
            &discussion,
            format!("Hi, @{}!", user.username),
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab as gl;
    use crate::interface::ParseAtom;
    use crate::utils::for_tests::*;

    #[tokio::test(threaded_scheduler)]
    async fn responds_hi() {
        let ctxt = TaskContext::mock().await;

        let user_mock = mock_default_user();

        let note_mock = mock_note_created(
            gl::ProjectId::new(222),
            gl::MergeRequestIid::new(333),
            &gl::DiscussionId::new("cafebabe"),
            "Hi, @someone!",
        );

        handle_hi(
            ctxt.clone(),
            UserId::new(100),
            DiscussionId::new("cafebabe"),
            MergeRequestPtr::do_parse("222!333"),
        )
        .await
        .unwrap();

        user_mock.assert();
        note_mock.assert();
    }
}
