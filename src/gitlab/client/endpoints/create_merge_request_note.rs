use crate::gitlab::{DiscussionId, GitLabClient, MergeRequestIid, ProjectId};
use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Request {
    body: String,
}

impl GitLabClient {
    pub async fn create_merge_request_note(
        &self,
        project: ProjectId,
        merge_request: MergeRequestIid,
        discussion: &DiscussionId,
        note: impl AsRef<str>,
    ) -> Result<()> {
        let note = note.as_ref();

        log::trace!(
            "create_merge_request_note(); project={}, merge_request={}, discussion={}, note={}",
            project.inner(),
            merge_request.inner(),
            discussion.as_ref(),
            note,
        );

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("projects/")?
                .join(&format!("{}/", project.inner()))?
                .join("merge_requests/")?
                .join(&format!("{}/", merge_request.inner()))?
                .join("discussions/")?
                .join(&format!("{}/", discussion.as_ref()))?
                .join("notes")?;

            let request = Request { body: note.into() };

            self.client
                .post(url)
                .json(&request)
                .send()
                .await?
                .error_for_status()?;
        }: Result<()>)
            .context("Couldn't create merge request note")
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
