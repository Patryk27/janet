use lib_e2e::*;

mod when_user_adds_new_reminder {
    use super::*;

    mod and_that_time_passes {
        use super::*;
        use tokio::time::{delay_for, Duration};

        #[tokio::test(threaded_scheduler)]
        async fn reminds() {
            test(async move |ctxt| {
                ctxt.gitlab.expect_user(&gl_mock::user_250()).await;
                ctxt.gitlab.expect_project(&gl_mock::project_10()).await;

                ctxt.gitlab
                    .expect_merge_request(&gl_mock::merge_request_100())
                    .await;

                ctxt.gitlab
                    .expect_merge_request_note_created(
                        gl::ProjectId::new(10),
                        gl::MergeRequestIid::new(1),
                        &gl::DiscussionId::new("cafebabe"),
                        "@someone :+1:",
                    )
                    .await;

                ctxt.gitlab
                    .expect_merge_request_note_created(
                        gl::ProjectId::new(10),
                        gl::MergeRequestIid::new(1),
                        &gl::DiscussionId::new("cafebabe"),
                        "@someone reminding: works!",
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
                            "description": "@janet remind me in 0s: works!",
                            "discussion_id": "cafebabe",
                        },
                    }))
                    .await;

                delay_for(Duration::from_secs(5)).await;
            })
            .await;
        }
    }
}
