use crate::gitlab::{GitLabClient, Project};
use anyhow::{Context, Result};

impl GitLabClient {
    pub async fn project(&self, id: impl AsRef<str>) -> Result<Project> {
        let id = id.as_ref();

        log::trace!("project(); id={}", id);

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
