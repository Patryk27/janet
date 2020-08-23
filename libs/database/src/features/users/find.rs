use crate::features::prelude::*;
use crate::User;

#[derive(Clone, Debug, Default)]
pub struct FindUsers {
    /// Internal user id
    pub id: Option<Id<User>>,

    /// GitLab's user id
    pub ext_id: Option<gl::UserId>,
}

impl FindUsers {
    pub fn id(id: Id<User>) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn ext_id(ext_id: gl::UserId) -> Self {
        Self {
            ext_id: Some(ext_id),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Query for FindUsers {
    type Model = User;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Finding users");

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
            .with_context(|| format!("Couldn't find users for query: {:?}", self))
    }
}
