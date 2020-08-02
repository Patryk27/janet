use crate::gitlab::{GitLabClient, NamespaceId};
use crate::interface::NamespacePtr;
use anyhow::*;

impl NamespacePtr {
    pub async fn resolve(&self, gitlab: &GitLabClient) -> Result<NamespaceId> {
        tracing::debug!("Resolving namespace ptr: {:?}", self);

        (try {
            match self {
                Self::Id(id) => *id,
                Self::Name(name) => gitlab.namespace(name.as_ref().into()).await?.id,
            }
        }: Result<_>)
            .with_context(|| format!("Couldn't resolve namespace ptr: {:?}", self))
    }
}
