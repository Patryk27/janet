use crate::{GitLabClient, Project};
use anyhow::*;

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn project(&self, id: &str) -> Result<Project> {
        tracing::debug!("Sending request");

        (try {
            let id = id.replace("/", "%2f");

            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("projects/")?
                .join(&id)?;

            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?
        }: Result<_>)
            .map_err(|err| {
                tracing::warn!({ err = ?err }, "Couldn't find project");
                err
            })
            .with_context(|| format!("Couldn't find project: {}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions as pa;

    mod given_existing_project {
        use super::*;
        use crate::mock::project_10;

        #[tokio::test(threaded_scheduler)]
        async fn returns_it() {
            let (server, client) = GitLabClient::mock().await;
            let expected = project_10();

            server.expect_project(&expected).await;

            let actual = client.project("10").await.unwrap();

            pa::assert_eq!(expected, actual);
        }
    }
}
