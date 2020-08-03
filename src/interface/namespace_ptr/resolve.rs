use crate::gitlab::{GitLabClient, NamespaceId};
use crate::interface::NamespacePtr;
use anyhow::*;

impl NamespacePtr {
    #[tracing::instrument(skip(gitlab))]
    pub async fn resolve(&self, gitlab: &GitLabClient) -> Result<NamespaceId> {
        tracing::debug!("Resolving namespace pointer");

        (try {
            match self {
                Self::Id(id) => *id,
                Self::Name(name) => gitlab.namespace(name.as_ref()).await?.id,
            }
        }: Result<_>)
            .with_context(|| format!("Couldn't resolve namespace ptr: {:?}", self))
    }
}
