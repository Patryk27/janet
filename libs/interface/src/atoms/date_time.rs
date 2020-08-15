use crate::{Date, Time};
use serde::Serialize;

mod atom;
mod resolve;

/// A "raw" date-time, as written by the user, e.g. `in 3 days at 21:37`.
///
/// This structure exposes a `.resolve()` function that allows to transform it
/// into a specific date-time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct DateTime {
    pub date: Option<Date>,
    pub time: Option<Time>,
}
