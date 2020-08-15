use crate::RelativeTime;
use chrono::NaiveTime;
use serde::Serialize;

mod atom;
mod resolve;

/// A time, e.g. `at 12:00` or `in 3h`.
///
/// Used as a part of the `time` component of the `DateTime` atom.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Time {
    /// E.g. `at 12:00`
    Absolute(NaiveTime),

    /// E.g. `in 3h`
    Relative(RelativeTime),
}
