use crate::{GitLabClient, User, UserId};
use anyhow::*;

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn user(&self, id: UserId) -> Result<User> {
        tracing::debug!("Sending request");

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("users/")?
                .join(&id.inner().to_string())?;

            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?
        }: Result<_>)
            .map_err(|err| {
                tracing::warn!({ err = ?err }, "Couldn't find user");
                err
            })
            .with_context(|| format!("Couldn't find user: {}", id.inner()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions as pa;

    mod given_existing_user {
        use super::*;
        use crate::mock::user_250;

        #[tokio::test(threaded_scheduler)]
        async fn returns_it() {
            let (server, client) = GitLabClient::mock().await;
            let expected = user_250();

            server.expect_user(&expected).await;

            let actual = client.user(UserId::new(250)).await.unwrap();

            pa::assert_eq!(expected, actual);
        }
    }
}
