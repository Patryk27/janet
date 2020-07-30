use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MergeRequestId(usize);

impl MergeRequestId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
