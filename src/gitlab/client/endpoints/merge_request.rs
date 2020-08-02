use crate::gitlab::{GitLabClient, MergeRequest, MergeRequestIid, ProjectId};
use anyhow::{Context, Result};

impl GitLabClient {
    pub async fn merge_request(
        &self,
        project: ProjectId,
        merge_request: MergeRequestIid,
    ) -> Result<MergeRequest> {
        let project = project.inner();
        let merge_request = merge_request.inner();

        log::trace!(
            "merge_request(); project={}, merge_request={}",
            project,
            merge_request
        );

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("projects/")?
                .join(&format!("{}/", project))?
                .join("merge_requests/")?
                .join(&merge_request.to_string())?;

            self.client
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?
        }: Result<MergeRequest>)
            .with_context(|| {
                format!(
                    "Couldn't find merge request: project={}, merge_request={}",
                    project, merge_request
                )
            })
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
