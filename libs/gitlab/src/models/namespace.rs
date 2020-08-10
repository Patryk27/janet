use crate::{NamespaceId, NamespaceName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Namespace {
    pub id: NamespaceId,
    pub name: NamespaceName,
    pub full_path: String,
}
