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
    use crate::gitlab::User;
    use crate::interface::Parse;
    use crate::utils::to_json;

    #[tokio::test(threaded_scheduler)]
    async fn responds_hi() {
        let ctxt = TaskContext::mock().await;

        let user_mock = mockito::mock("GET", "/gitlab/api/v4/users/123")
            .with_body(to_json(&User {
                id: UserId::new(123),
                username: "someone".into(),
            }))
            .create();

        let note_mock = mockito::mock(
            "POST",
            "/gitlab/api/v4/projects/222/merge_requests/333/discussions/cafebabe/notes",
        )
        .match_body(r#"{"body":"Hi, @someone!"}"#)
        .create();

        handle_hi(
            ctxt.clone(),
            UserId::new(123),
            DiscussionId::new("cafebabe"),
            MergeRequestPtr::do_parse("222!333"),
        )
        .await
        .unwrap();

        user_mock.assert();
        note_mock.assert();
    }
}
