use crate::features::prelude::*;
use crate::{MergeRequest, MergeRequestDependency, User};

#[derive(Clone, Debug, Default)]
pub struct GetMergeRequestDependencies<'a> {
    /// Internal dependency id
    pub id: Option<Id<MergeRequestDependency>>,

    /// Internal id of the user who should be notified on change
    pub user_id: Option<Id<User>>,

    /// GitLab's discussion id
    pub ext_discussion_id: Option<&'a gl::DiscussionId>,

    /// Internal id of the source merge request (i.e. the one where you write
    /// the `depends on` comment)
    pub src_merge_request_id: Option<Id<MergeRequest>>,

    /// Internal id of the destination merge request (i.e. the one referred
    /// inside the `depends on` comment)
    pub dst_merge_request_id: Option<Id<MergeRequest>>,
}

#[async_trait]
impl Query for GetMergeRequestDependencies<'_> {
    type Model = MergeRequestDependency;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Searching for merge request dependencies");

        let mut query = String::from("SELECT * FROM merge_request_dependencies WHERE 1 = 1");
        let mut args = SqliteArguments::default();

        if let Some(id) = self.id {
            query += " AND id = ?";
            args.add(id);
        }

        if let Some(user_id) = self.user_id {
            query += " AND user_id = ?";
            args.add(user_id);
        }

        if let Some(ext_discussion_id) = self.ext_discussion_id {
            query += " AND ext_discussion_id = ?";
            args.add(ext_discussion_id.as_ref());
        }

        if let Some(src_merge_request_id) = self.src_merge_request_id {
            query += " AND src_merge_request_id = ?";
            args.add(src_merge_request_id);
        }

        if let Some(dst_merge_request_id) = self.dst_merge_request_id {
            query += " AND dst_merge_request_id = ?";
            args.add(dst_merge_request_id);
        }

        sqlx::query_as_with(&query, args)
            .fetch_all(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't search for merge request dependencies: {:?}", self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_merge_request, create_project, create_user};
    use crate::{CreateMergeRequestDependency, Project};
    use std::collections::BTreeSet;

    #[allow(dead_code)]
    struct TestContext {
        db: Database,
        project_10: Id<Project>,
        project_11: Id<Project>,
        merge_request_100: Id<MergeRequest>,
        merge_request_101: Id<MergeRequest>,
        merge_request_102: Id<MergeRequest>,
        user_250: Id<User>,
        user_251: Id<User>,
        id_1: Id<MergeRequestDependency>,
        id_2: Id<MergeRequestDependency>,
    }

    impl TestContext {
        async fn new() -> Self {
            let db = Database::mock().await;

            let project_10 = create_project(&db, 10).await;
            let project_11 = create_project(&db, 11).await;

            let merge_request_100 = create_merge_request(&db, project_10, 100, 1).await;
            let merge_request_101 = create_merge_request(&db, project_10, 101, 2).await;
            let merge_request_102 = create_merge_request(&db, project_11, 102, 3).await;

            let user_250 = create_user(&db, 250).await;
            let user_251 = create_user(&db, 251).await;

            let id_1 = db
                .execute(CreateMergeRequestDependency {
                    user_id: user_250,
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    src_merge_request_id: merge_request_100,
                    dst_merge_request_id: merge_request_101,
                })
                .await
                .unwrap();

            let id_2 = db
                .execute(CreateMergeRequestDependency {
                    user_id: user_251,
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    src_merge_request_id: merge_request_101,
                    dst_merge_request_id: merge_request_102,
                })
                .await
                .unwrap();

            Self {
                db,
                project_10,
                project_11,
                merge_request_100,
                merge_request_101,
                merge_request_102,
                user_250,
                user_251,
                id_1,
                id_2,
            }
        }

        async fn assert_ids(
            &self,
            query: GetMergeRequestDependencies<'_>,
            expected_ids: &[Id<MergeRequestDependency>],
        ) {
            let actual_ids: BTreeSet<_> = self
                .db
                .find_all(query)
                .await
                .unwrap()
                .into_iter()
                .map(|dep| dep.id)
                .collect();

            let expected_ids: BTreeSet<_> = expected_ids.iter().cloned().collect();

            assert_eq!(expected_ids, actual_ids);
        }
    }

    mod given_empty_filter {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_all_items() {
            let ctxt = TestContext::new().await;
            let query = GetMergeRequestDependencies::default();

            ctxt.assert_ids(query, &[ctxt.id_1, ctxt.id_2]).await;
        }
    }

    mod given_filter_with_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_matching_items() {
            let ctxt = TestContext::new().await;

            {
                let query = GetMergeRequestDependencies {
                    id: Some(ctxt.id_1),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_1]).await;
            }

            {
                let query = GetMergeRequestDependencies {
                    id: Some(ctxt.id_2),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_2]).await;
            }
        }
    }

    mod given_filter_with_user_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_matching_items() {
            let ctxt = TestContext::new().await;

            {
                let query = GetMergeRequestDependencies {
                    user_id: Some(ctxt.user_250),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_1]).await;
            }

            {
                let query = GetMergeRequestDependencies {
                    user_id: Some(ctxt.user_251),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_2]).await;
            }
        }
    }

    mod given_filter_with_ext_discussion_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_matching_items() {
            let ctxt = TestContext::new().await;

            {
                let ext_discussion_id = gl::DiscussionId::new("cafebabe");

                let query = GetMergeRequestDependencies {
                    ext_discussion_id: Some(&ext_discussion_id),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_1, ctxt.id_2]).await;
            }

            {
                let ext_discussion_id = gl::DiscussionId::new("CAFEBABE");

                let query = GetMergeRequestDependencies {
                    ext_discussion_id: Some(&ext_discussion_id),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[]).await;
            }
        }
    }

    mod given_filter_with_src_merge_request_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_matching_items() {
            let ctxt = TestContext::new().await;

            {
                let query = GetMergeRequestDependencies {
                    src_merge_request_id: Some(ctxt.merge_request_100),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_1]).await;
            }

            {
                let query = GetMergeRequestDependencies {
                    src_merge_request_id: Some(ctxt.merge_request_101),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_2]).await;
            }
        }
    }

    mod given_filter_with_dst_merge_request_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_matching_items() {
            let ctxt = TestContext::new().await;

            {
                let query = GetMergeRequestDependencies {
                    dst_merge_request_id: Some(ctxt.merge_request_100),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[]).await;
            }

            {
                let query = GetMergeRequestDependencies {
                    dst_merge_request_id: Some(ctxt.merge_request_101),
                    ..Default::default()
                };

                ctxt.assert_ids(query, &[ctxt.id_1]).await;
            }
        }
    }
}
