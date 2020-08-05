use crate::gitlab::{GitLabClient, MergeRequestIid, ProjectId};
use crate::interface::{MergeRequestPtr, PtrContext};
use anyhow::*;
use url::Url;

impl MergeRequestPtr {
    #[tracing::instrument(skip(gitlab))]
    pub async fn resolve(
        &self,
        gitlab: &GitLabClient,
        ctxt: &PtrContext,
    ) -> Result<(ProjectId, MergeRequestIid)> {
        tracing::debug!("Resolving merge request pointer");

        (try {
            match self {
                Self::Iid {
                    project,
                    merge_request,
                } => {
                    let project = if let Some(project) = project {
                        project.resolve(gitlab, ctxt).await?
                    } else {
                        ctxt.project_id
                            .ok_or_else(|| anyhow!("Cannot infer project id"))?
                    };

                    (project, *merge_request)
                }

                Self::Url(url) => {
                    let url = url.path().to_lowercase();
                    let merge_requests = gitlab.merge_requests().await?;

                    // This is suboptimal at best, but seems like GitLab doesn't allow to find merge
                    // request by web_url via API
                    //
                    // TODO check if this is really the only reliable way to approach this
                    for merge_request in merge_requests {
                        if let Ok(mr_url) = Url::parse(&merge_request.web_url) {
                            if mr_url.path().to_lowercase() == url {
                                return Ok((merge_request.project_id, merge_request.iid));
                            }
                        }
                    }

                    bail!("Found mo merge requests matching given URL");
                }
            }
        }: Result<_>)
            .with_context(|| format!("Couldn't resolve merge request ptr: {:?}", self))
    }
}
