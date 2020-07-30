use crate::gitlab::{GitLabClient, MergeRequestIid, ProjectId};
use crate::interface::{ProjectPtr, PtrContext};
use anyhow::{anyhow, Context, Result};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum MergeRequestPtr {
    Iid {
        project: Option<ProjectPtr>,
        merge_request: MergeRequestIid,
    },

    Url(String),
}

impl MergeRequestPtr {
    pub async fn resolve(
        &self,
        gitlab: &GitLabClient,
        ctxt: &PtrContext,
    ) -> Result<(ProjectId, MergeRequestIid)> {
        log::debug!("Resolving merge request ptr: {:?}", self);

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
