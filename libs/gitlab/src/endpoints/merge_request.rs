use crate::{GitLabClient, MergeRequest, MergeRequestIid, ProjectId};
use anyhow::*;

impl GitLabClient {
    #[tracing::instrument(skip(self))]
    pub async fn merge_request(
        &self,
        project: ProjectId,
        merge_request: MergeRequestIid,
    ) -> Result<MergeRequest> {
        tracing::debug!("Sending request");

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("projects/")?
                .join(&format!("{}/", project.inner()))?
                .join("merge_requests/")?
                .join(&merge_request.inner().to_string())?;

            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?
        }: Result<_>)
            .map_err(|err| {
                tracing::warn!({ err = ?err }, "Couldn't find merge request");
                err
            })
            .with_context(|| {
                format!(
                    "Couldn't find merge request: project={}, merge_request={}",
                    project.inner(),
                    merge_request.inner()
                )
            })
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
