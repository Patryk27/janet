use serde::Serialize;

mod parse;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(name.as_ref().into())
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
