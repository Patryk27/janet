use crate::gitlab::{GitLabClient, MergeRequestIid, ProjectId};
use crate::interface::{MergeRequestPtr, PtrContext};
use anyhow::*;

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

                Self::Url(url) => todo!(),
            }
        }: Result<_>)
            .with_context(|| format!("Couldn't resolve merge request ptr: {:?}", self))
    }
}
