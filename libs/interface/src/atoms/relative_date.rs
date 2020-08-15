use crate::DayOfWeek;
use serde::Serialize;

mod atom;
mod resolve;

/// A relative date, e.g. `the day after tomorrow`.
///
/// Used as a part of the `date` component of the `DateTime` atom.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum RelativeDate {
    /// Today + `n` days; e.g.:
    ///
    /// - `today` as `Days(0)`,
    /// - `tomorrow` as `Days(1)`,
    /// - `the day after tomorrow` as `Days(2)`,
    /// - `in 123 days` as `Days(123)`.
    Days(usize),

    /// Nearest day of week; e.g.:
    ///
    /// - `monday`,
    /// - `friday`.
    DayOfWeek(DayOfWeek),

    /// Today + `n` weeks; e.g.:
    ///
    /// - `next week` as `Weeks(1)`,
    /// - `in 1 week` as `Weeks(1)`,
    /// - `in 2 weeks` as `Weeks(2)`.
    Weeks(usize),
}
