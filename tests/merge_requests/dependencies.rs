use lib_e2e::*;

mod when_user_adds_comment {
    use super::*;

    mod that_refers_to_existing_merge_request {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn responds_with_acknowledgement() {
            test(async move |ctxt| {
                ctxt.gitlab.expect_user(1, &gl_mock::user_250()).await;
                ctxt.gitlab.expect_project(2, &gl_mock::project_10()).await;

                ctxt.gitlab
                    .expect_merge_request(1, &gl_mock::merge_request_100())
                    .await;

                ctxt.gitlab
                    .expect_merge_request(2, &gl_mock::merge_request_101())
                    .await;

                ctxt.gitlab
                    .expect_merge_request_note_created(
                        gl::ProjectId::new(10),
                        gl::MergeRequestIid::new(1),
                        &gl::DiscussionId::new("cafebabe"),
                        "@someone :+1:",
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
                            "description": "@janet depends on !2",
                            "discussion_id": "cafebabe",
                        },
                    }))
                    .await;
            })
            .await;
        }
    }

    mod that_refers_to_missing_merge_request {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn responds_with_error() {
            test(async move |ctxt| {
                ctxt.gitlab.expect_user(2, &gl_mock::user_250()).await;
                ctxt.gitlab.expect_project(1, &gl_mock::project_10()).await;

                ctxt.gitlab
                    .expect_merge_request(1, &gl_mock::merge_request_100())
                    .await;

                ctxt.gitlab
                    .expect_merge_request_note_created(
                        gl::ProjectId::new(10),
                        gl::MergeRequestIid::new(1),
                        &gl::DiscussionId::new("cafebabe"),
                        "@someone sorry, I couldn't find this merge request - could you please ensure it exists and re-create your comment?",
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
                                "description": "@janet depends on !33",
                                "discussion_id": "cafebabe",
                            },
                        }))
                    .await;
            }).await;
        }
    }
}
