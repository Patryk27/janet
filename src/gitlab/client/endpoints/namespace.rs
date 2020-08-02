use crate::gitlab::{GitLabClient, Namespace};
use anyhow::{Context, Result};

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn namespace(&self, id: String) -> Result<Namespace> {
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
        }: Result<Namespace>)
            .with_context(|| format!("Couldn't find namespace: {}", id))
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
