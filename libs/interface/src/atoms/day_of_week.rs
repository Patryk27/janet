use serde::Serialize;

mod atom;

/// A day of week, e.g. `sunday`.
///
/// Used as a part of the `date` component of the `DateTime` atom.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    /// Returns ISO 8601 weekday number.
    pub fn number_from_monday(self) -> u32 {
        match self {
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
            Self::Sunday => 7,
        }
    }
}
