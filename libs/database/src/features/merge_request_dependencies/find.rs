use crate::features::prelude::*;
use crate::{MergeRequest, MergeRequestDependency, User};

#[derive(Clone, Debug, Default)]
pub struct FindMergeRequestDependencies<'a> {
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
impl Query for FindMergeRequestDependencies<'_> {
    type Model = MergeRequestDependency;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Finding merge request dependencies");

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
            .with_context(|| {
                format!(
                    "Couldn't find merge request dependencies for query: {:?}",
                    self
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_merge_request, create_project, create_user};
    use crate::CreateMergeRequestDependency;
    use std::collections::BTreeSet;

    struct TestContext {
        db: Database,
        merge_requests: [Id<MergeRequest>; 3],
        users: [Id<User>; 2],
        deps: [Id<MergeRequestDependency>; 2],
    }

    impl TestContext {
        async fn new() -> Self {
            let db = Database::mock().await;

            let projects = [create_project(&db, 1).await, create_project(&db, 2).await];

            let merge_requests = [
                create_merge_request(&db, projects[0], 1, 10).await,
                create_merge_request(&db, projects[0], 2, 20).await,
                create_merge_request(&db, projects[1], 3, 30).await,
            ];

            let users = [create_user(&db, 1).await, create_user(&db, 2).await];

            let dep_1 = db
                .execute(CreateMergeRequestDependency {
                    user_id: users[0],
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    src_merge_request_id: merge_requests[0],
                    dst_merge_request_id: merge_requests[1],
                })
                .await
                .unwrap();

            let dep_2 = db
                .execute(CreateMergeRequestDependency {
                    user_id: users[1],
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    src_merge_request_id: merge_requests[1],
                    dst_merge_request_id: merge_requests[2],
                })
                .await
                .unwrap();

            Self {
                db,
                merge_requests,
                users,
                deps: [dep_1, dep_2],
            }
        }

        async fn assert_query_returns(
            &self,
            query: FindMergeRequestDependencies<'_>,
            expected: &[Id<MergeRequestDependency>],
        ) -> Result<()> {
            let actual: BTreeSet<_> = self
                .db
                .get_all(query)
                .await
                .unwrap()
                .into_iter()
                .map(|dep| dep.id)
                .collect();

            let expected: BTreeSet<_> = expected.iter().cloned().collect();

            if actual == expected {
                Ok(())
            } else {
                bail!(
                    "Query returned different result set;\nactual={:?}\nexpected={:?}",
                    actual,
                    expected
                )
            }
        }
    }

    mod given_empty_filter {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_all_items() {
            let ctxt = TestContext::new().await;
            let query = FindMergeRequestDependencies::default();

            ctxt.assert_query_returns(query, &ctxt.deps).await.unwrap();
        }
    }

    mod given_filter_with_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_dependency_with_given_id() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                (ctxt.deps[0], vec![ctxt.deps[0]]),
                (ctxt.deps[1], vec![ctxt.deps[1]]),
            ];

            for (case_idx, (id, expected)) in cases.into_iter().enumerate() {
                let query = FindMergeRequestDependencies {
                    id: Some(id),
                    ..Default::default()
                };

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }

    mod given_filter_with_user_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_dependencies_for_given_user() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                (ctxt.users[0], vec![ctxt.deps[0]]),
                (ctxt.users[1], vec![ctxt.deps[1]]),
            ];

            for (case_idx, (user_id, expected)) in cases.into_iter().enumerate() {
                let query = FindMergeRequestDependencies {
                    user_id: Some(user_id),
                    ..Default::default()
                };

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }

    mod given_filter_with_ext_discussion_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_dependencies_for_given_discussion() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                ("cafebabe", vec![ctxt.deps[0], ctxt.deps[1]]),
                ("CAFEBABE", vec![]),
                (" cafebabe ", vec![]),
                ("test test", vec![]),
            ];

            for (case_idx, (ext_discussion_id, expected)) in cases.into_iter().enumerate() {
                let ext_discussion_id = gl::DiscussionId::new(ext_discussion_id);

                let query = FindMergeRequestDependencies {
                    ext_discussion_id: Some(&ext_discussion_id),
                    ..Default::default()
                };

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }

    mod given_filter_with_src_merge_request_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_dependencies_for_given_source_merge_request() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                (ctxt.merge_requests[0], vec![ctxt.deps[0]]),
                (ctxt.merge_requests[1], vec![ctxt.deps[1]]),
                (ctxt.merge_requests[2], vec![]),
            ];

            for (case_idx, (src_merge_request_id, expected)) in cases.into_iter().enumerate() {
                let query = FindMergeRequestDependencies {
                    src_merge_request_id: Some(src_merge_request_id),
                    ..Default::default()
                };

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }

    mod given_filter_with_dst_merge_request_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_dependencies_for_given_destination_merge_request() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                (ctxt.merge_requests[0], vec![]),
                (ctxt.merge_requests[1], vec![ctxt.deps[0]]),
                (ctxt.merge_requests[2], vec![ctxt.deps[1]]),
            ];

            for (case_idx, (dst_merge_request_id, expected)) in cases.into_iter().enumerate() {
                let query = FindMergeRequestDependencies {
                    dst_merge_request_id: Some(dst_merge_request_id),
                    ..Default::default()
                };

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }
}
