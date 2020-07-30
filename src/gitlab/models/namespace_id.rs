use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NamespaceId(usize);

impl NamespaceId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> usize {
        self.0
    }
}
