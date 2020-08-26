use crate::{GitLabClient, Namespace};
use anyhow::*;

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn namespace(&self, id: &str) -> Result<Namespace> {
        tracing::debug!("Sending request");

        (try {
            let id = id.replace("/", "%2f");

            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("namespaces/")?
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
                tracing::warn!({ err = ?err }, "Couldn't find namespace");
                err
            })
            .with_context(|| format!("Couldn't find namespace: {}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions as pa;

    mod given_existing_namespace {
        use super::*;
        use crate::mock::namespace_1;

        #[tokio::test(threaded_scheduler)]
        async fn returns_it() {
            let (server, client) = GitLabClient::mock().await;
            let expected = namespace_1();

            server.expect_namespace(&expected).await;

            let actual = client.namespace("1").await.unwrap();

            pa::assert_eq!(expected, actual);
        }
    }
}
