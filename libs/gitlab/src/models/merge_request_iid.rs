use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MergeRequestIid(usize);

impl MergeRequestIid {
    pub fn new(iid: usize) -> Self {
        Self(iid)
    }

    pub fn inner(&self) -> usize {
        self.0
    }
}
