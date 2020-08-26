use serde::Serialize;

mod atom;
mod resolve;

/// A relative time, e.g. `3h`.
///
/// Used as a part of the `time` component of the `DateTime` atom.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RelativeTime {
    pub hours: Option<usize>,
    pub minutes: Option<usize>,
    pub seconds: Option<usize>,
}
