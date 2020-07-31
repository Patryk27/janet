use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NamespaceName(String);

impl NamespaceName {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(name.as_ref().into())
    }
}

impl AsRef<str> for NamespaceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
