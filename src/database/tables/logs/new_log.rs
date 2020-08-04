use crate::interface::{Command, Event};

#[derive(Clone, Debug)]
pub struct NewLog {
    pub event: String,
    pub payload: String,
}

impl From<&Command> for NewLog {
    fn from(cmd: &Command) -> Self {
        Self {
            event: "command".into(),
            payload: serde_json::to_string(cmd).unwrap(),
        }
    }
}

impl From<&Event> for NewLog {
    fn from(evt: &Event) -> Self {
        Self {
            event: "event".into(),
            payload: serde_json::to_string(evt).unwrap(),
        }
    }
}
