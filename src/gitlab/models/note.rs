use crate::gitlab::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    pub author_id: UserId,
    pub description: String,
}
