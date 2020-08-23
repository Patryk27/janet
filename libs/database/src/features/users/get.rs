use crate::features::prelude::*;
use crate::User;

#[derive(Clone, Debug, Default)]
pub struct GetUsers {
    /// Internal user id
    pub id: Option<Id<User>>,

    /// GitLab's user id
    pub ext_id: Option<gl::UserId>,
}

#[async_trait]
impl Query for GetUsers {
    type Model = User;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Searching for users");

        let mut query = String::from("SELECT * FROM users WHERE 1 = 1");
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
            .with_context(|| format!("Couldn't search for users: {:?}", self))
    }
}
