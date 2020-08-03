use crate::gitlab::{GitLabClient, User, UserId};
use anyhow::{Context, Result};

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
            .with_context(|| format!("Couldn't find user: {}", id.inner()))
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
