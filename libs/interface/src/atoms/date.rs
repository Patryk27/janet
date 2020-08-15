use crate::RelativeDate;
use chrono::NaiveDate;
use serde::Serialize;

mod atom;
mod resolve;

/// A date, e.g. `2018-01-01`, `monday` or `in 3 days`.
///
/// Used as a part of the `date` component of the `DateTime` atom.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Date {
    /// E.g. `2018-01-01`
    Absolute(NaiveDate),

    /// E.g. `monday` or `in 3 days`
    Relative(RelativeDate),
}
