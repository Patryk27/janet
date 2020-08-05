pub use self::{new_user::*, user::*};

mod new_user;
mod user;

use crate::database::{Database, Id};
use anyhow::*;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct UsersRepository {
    db: Database,
}

impl UsersRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    #[tracing::instrument(skip(self))]
    pub async fn add(&self, user: &NewUser) -> Result<Id<User>> {
        if let Some(id) = self.find_by_new(user).await? {
            return Ok(id);
        }

        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;
        let id = Id::new();

        sqlx::query(
            "
            INSERT INTO users (
                id,
                ext_id
            )
            VALUES (?, ?)
            ",
        )
        .bind(id)
        .bind(user.ext_id)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't create user: {:?}", user))?;

        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get(&self, id: Id<User>) -> Result<User> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't load user: {}", id))
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_new(&self, user: &NewUser) -> Result<Option<Id<User>>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                id

            FROM
                users

            WHERE
                ext_id = ?
            ",
        )
        .bind(user.ext_id)
        .fetch_optional(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't find user: {:?}", user))
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
                let user = NewUser { ext_id: i };

                let id = db.users().add(&user).await.unwrap();
                let id2 = db.users().add(&user).await.unwrap();

                assert_eq!(id2, id);

                ids.push(id);
            }

            for (i, id) in ids.into_iter().enumerate() {
                let i = i as i64;
                let user = db.users().get(id).await.unwrap();

                assert_eq!(id, user.id);
                assert_eq!(i, user.ext_id);
            }
        }
    }
}
