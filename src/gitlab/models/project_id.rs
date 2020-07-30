use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ProjectId(usize);

impl ProjectId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
