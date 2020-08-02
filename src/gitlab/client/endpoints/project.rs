use crate::gitlab::{GitLabClient, Project};
use anyhow::{Context, Result};

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn project(&self, id: String) -> Result<Project> {
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
        }: Result<Project>)
            .with_context(|| format!("Couldn't find project: {}", id))
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
