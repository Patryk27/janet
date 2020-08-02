use serde::Serialize;

mod parse;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Id(usize);

impl Id {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}
