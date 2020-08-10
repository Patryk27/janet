use crate::{GitLabClient, MergeRequest};
use anyhow::*;

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn merge_requests(&self) -> Result<Vec<MergeRequest>> {
        tracing::debug!("Sending request");

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("merge_requests?scope=all")?;

            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?
        }: Result<_>)
            .map_err(|err| {
                tracing::warn!({ err = ?err }, "Couldn't find merge requests");
                err
            })
            .context("Couldn't find merge requests")
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
