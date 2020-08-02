use crate::interface::DateTimeSpec;
use chrono::{DateTime, Utc};

impl DateTimeSpec {
    pub fn resolve(&self, now: DateTime<Utc>) -> DateTime<Utc> {
        todo!()
    }
}
