use serde::Serialize;

mod parse;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Url(String);

impl Url {
    pub fn new(url: impl AsRef<str>) -> Self {
        Self(url.as_ref().into())
    }
}

impl Into<String> for Url {
    fn into(self) -> String {
        self.0
    }
}
