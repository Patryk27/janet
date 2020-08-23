use crate::features::prelude::*;
use crate::MergeRequest;

#[derive(Clone, Debug, Default)]
pub struct GetMergeRequests {
    /// Internal merge request id
    pub id: Option<Id<MergeRequest>>,

    /// GitLab's merge request id
    pub ext_id: Option<gl::MergeRequestId>,

    /// GitLab's merge request incremental id
    pub ext_iid: Option<gl::MergeRequestIid>,

    /// GitLab's project id
    pub ext_project_id: Option<gl::ProjectId>,
}

#[async_trait]
impl Query for GetMergeRequests {
    type Model = MergeRequest;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Searching for merge requests");

        let mut query = String::from(
            "
             SELECT
                mr.*

             FROM
                merge_requests mr

             INNER JOIN
                 projects p ON p.id = mr.project_id

             WHERE
                1 = 1
            ",
        );

        let mut args = SqliteArguments::default();

        if let Some(id) = self.id {
            query += " AND mr.id = ?";
            args.add(id);
        }

        if let Some(ext_id) = self.ext_id {
            query += " AND mr.ext_id = ?";
            args.add(ext_id.inner() as i64);
        }

        if let Some(ext_iid) = self.ext_iid {
            query += " AND mr.ext_iid = ?";
            args.add(ext_iid.inner() as i64);
        }

        if let Some(ext_project_id) = self.ext_project_id {
            query += " AND p.ext_id = ?";
            args.add(ext_project_id.inner() as i64);
        }

        sqlx::query_as_with(&query, args)
            .fetch_all(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't search for merge requests: {:?}", self))
    }
}
