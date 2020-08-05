use crate::gitlab::{NamespaceId, NamespaceName};
use serde::Serialize;

mod resolve;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum NamespacePtr {
    Id(NamespaceId),
    Name(NamespaceName),
}
