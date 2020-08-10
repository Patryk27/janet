use crate::GitLabClient;
use anyhow::*;

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn ping(&self) -> Result<()> {
        tracing::debug!("Sending request");

        (try {
            self.client
                .get(self.url.clone())
                .send()
                .await?
                .error_for_status()?;
        }: Result<_>)
            .map_err(|err| {
                tracing::warn!({ err = ?err }, "Couldn't ping");
                err
            })
            .with_context(|| format!("Couldn't ping GitLab at: {}", self.url))
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
