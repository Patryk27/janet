use crate::gitlab::{NamespaceId, NamespaceName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Namespace {
    pub id: NamespaceId,
    pub name: NamespaceName,
    pub full_path: String,
}
