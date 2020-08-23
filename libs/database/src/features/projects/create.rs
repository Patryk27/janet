use crate::features::prelude::*;
use crate::{GetProjects, Project};

#[derive(Clone, Debug)]
pub struct CreateProject {
    /// GitLab's project id
    pub ext_id: gl::ProjectId,
}

#[async_trait]
impl Command for CreateProject {
    type Output = Id<Project>;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        // Creating projects is idempotent - i.e. creating the same project for the
        // second time is a no-op
        if let Some(project) = db
            .maybe_find_one(GetProjects {
                ext_id: Some(self.ext_id),
                ..Default::default()
            })
            .await?
        {
            return Ok(project.id);
        }

        tracing::debug!("Creating project");

        let id = Id::default();

        sqlx::query("INSERT INTO projects (id, ext_id) VALUES (?, ?)")
            .bind(id)
            .bind(self.ext_id.inner() as i64)
            .execute(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't create project: {:?}", self))?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(threaded_scheduler)]
    async fn test() {
        let db = Database::mock().await;

        let id = db
            .execute(CreateProject {
                ext_id: gl::ProjectId::new(123),
            })
            .await
            .unwrap();

        let project = db
            .find_one(GetProjects {
                id: Some(id),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(id, project.id);
        assert_eq!(123, project.ext_id as usize);
    }

    mod when_creating_the_same_project_for_the_second_time {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_already_existing_id() {
            let db = Database::mock().await;

            for i in 0..5 {
                let command = CreateProject {
                    ext_id: gl::ProjectId::new(i),
                };

                let id_1 = db.execute(command.clone()).await.unwrap();
                let id_2 = db.execute(command.clone()).await.unwrap();

                assert_eq!(id_1, id_2);
            }
        }
    }
}
