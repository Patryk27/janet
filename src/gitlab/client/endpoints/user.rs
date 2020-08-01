use crate::gitlab::{GitLabClient, User};
use anyhow::{Context, Result};

impl GitLabClient {
    pub async fn user(&self, id: impl AsRef<str>) -> Result<User> {
        let id = id.as_ref();

        log::trace!("user(); id={}", id);

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("users/")?
                .join(&id)?;

            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?
        }: Result<User>)
            .with_context(|| format!("Couldn't find user: {}", id))
    }
}
