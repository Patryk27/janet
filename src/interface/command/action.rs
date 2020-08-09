mod parse;

use serde::Serialize;

/// Some actions can be prepended with `+` or `-` (e.g. `-depends on`) - this
/// enum allows to distinguish which action has been meant by the user.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CommandAction {
    Add,
    Remove,
}

impl CommandAction {
    pub fn is_add(self) -> bool {
        self == Self::Add
    }

    pub fn is_remove(self) -> bool {
        self == Self::Remove
    }
}
