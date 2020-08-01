use crate::gitlab::{GitLabClient, NamespaceId, NamespaceName};
use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum NamespacePtr {
    Id(NamespaceId),
    Name(NamespaceName),
}

impl NamespacePtr {
    pub async fn resolve(&self, gitlab: &GitLabClient) -> Result<NamespaceId> {
        log::debug!("Resolving namespace ptr: {:?}", self);

        (try {
            match self {
                Self::Id(id) => *id,
                Self::Name(name) => gitlab.namespace(name).await?.id,
            }
        }: Result<_>)
            .with_context(|| format!("Couldn't resolve namespace ptr: {:?}", self))
    }
}
