use crate::features::prelude::*;
use crate::{FindUsers, User};

#[derive(Clone, Debug)]
pub struct CreateUser {
    /// GitLab's user id
    pub ext_id: gl::UserId,
}

#[async_trait]
impl Command for CreateUser {
    type Output = Id<User>;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        // Creating users is idempotent - i.e. creating the same user for the second
        // time is a no-op
        if let Some(user) = db.maybe_find_one(FindUsers::ext_id(self.ext_id)).await? {
            return Ok(user.id);
        }

        tracing::debug!("Creating user");

        let id = Id::default();

        sqlx::query("INSERT INTO users (id, ext_id) VALUES (?, ?)")
            .bind(id)
            .bind(self.ext_id.inner() as i64)
            .execute(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't create user: {:?}", self))?;

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
            .execute(CreateUser {
                ext_id: gl::UserId::new(123),
            })
            .await
            .unwrap();

        let user = db.find_one(FindUsers::id(id)).await.unwrap();

        assert_eq!(id, user.id);
        assert_eq!(123, user.ext_id as usize);
    }

    mod when_creating_the_same_user_for_the_second_time {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_already_existing_id() {
            let db = Database::mock().await;

            for i in 0..5 {
                let command = CreateUser {
                    ext_id: gl::UserId::new(i),
                };

                let id_1 = db.execute(command.clone()).await.unwrap();
                let id_2 = db.execute(command.clone()).await.unwrap();

                assert_eq!(id_1, id_2);
            }
        }
    }
}
