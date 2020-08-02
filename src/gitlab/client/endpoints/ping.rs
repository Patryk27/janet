use crate::gitlab::GitLabClient;
use anyhow::{Context, Result};

impl GitLabClient {
    pub async fn ping(&self) -> Result<()> {
        log::trace!("ping()");

        (try {
            self.client
                .get(self.url.clone())
                .send()
                .await?
                .error_for_status()?;
        }: Result<()>)
            .with_context(|| format!("Couldn't ping GitLab at: {}", self.url))
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
