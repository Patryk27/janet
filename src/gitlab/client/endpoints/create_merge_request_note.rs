use crate::gitlab::GitLabClient;
use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Request {
    body: String,
}

impl GitLabClient {
    pub async fn create_merge_request_note(
        &self,
        project: impl AsRef<str>,
        merge_request: impl AsRef<str>,
        note: impl AsRef<str>,
    ) -> Result<()> {
        let project = project.as_ref();
        let merge_request = merge_request.as_ref();
        let note = note.as_ref();

        log::trace!(
            "create_merge_request_note(); project={}, merge_request={}, note={}",
            project,
            merge_request,
            note,
        );

        (try {
            let url = self
                .url
                .join("api/")?
                .join("v4/")?
                .join("projects/")?
                .join(&format!("{}/", project))?
                .join("merge_requests/")?
                .join(&format!("{}/", merge_request))?
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

mod tests {
    // TODO
}
