pub use self::{new_project::*, project::*};

use crate::database::{Database, Id};
use anyhow::*;
use std::ops::DerefMut;

mod new_project;
mod project;

#[derive(Clone)]
pub struct ProjectsRepository {
    db: Database,
}

impl ProjectsRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    #[tracing::instrument(skip(self))]
    pub async fn add(self, project: &NewProject) -> Result<Id<Project>> {
        if let Some(id) = self.find_by_new(project).await? {
            return Ok(id);
        }

        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;
        let id = Id::new();

        sqlx::query(
            "
            INSERT INTO projects (
                id,
                ext_id
            )
            VALUES (?, ?)
            ",
        )
        .bind(id)
        .bind(project.ext_id)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't add project: {:?}", project))?;

        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get(&self, id: Id<Project>) -> Result<Project> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_one(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't load project: {}", id))
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_new(&self, project: &NewProject) -> Result<Option<Id<Project>>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                id

            FROM
                projects

            WHERE
                ext_id = ?
            ",
        )
        .bind(project.ext_id)
        .fetch_optional(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't find project: {:?}", project))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod add {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;

            let mut ids = Vec::new();

            for i in 0..10 {
                let project = NewProject { ext_id: i };

                let id = db.projects().add(&project).await.unwrap();
                let id2 = db.projects().add(&project).await.unwrap();

                assert_eq!(id2, id);

                ids.push(id);
            }

            for (i, id) in ids.into_iter().enumerate() {
                let i = i as i64;
                let project = db.projects().get(id).await.unwrap();

                assert_eq!(id, project.id);
                assert_eq!(i, project.ext_id);
            }
        }
    }
}
