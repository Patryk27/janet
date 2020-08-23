use crate::features::prelude::*;
use crate::Project;

#[derive(Clone, Debug, Default)]
pub struct GetProjects {
    /// Internal project id
    pub id: Option<Id<Project>>,

    /// GitLab's project id
    pub ext_id: Option<gl::ProjectId>,
}

#[async_trait]
impl Query for GetProjects {
    type Model = Project;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Searching for projects");

        let mut query = String::from("SELECT * FROM projects WHERE 1 = 1");
        let mut args = SqliteArguments::default();

        if let Some(id) = self.id {
            query += " AND id = ?";
            args.add(id);
        }

        if let Some(ext_id) = self.ext_id {
            query += " AND ext_id = ?";
            args.add(ext_id.inner() as i64);
        }

        sqlx::query_as_with(&query, args)
            .fetch_all(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't search for projects: {:?}", self))
    }
}
