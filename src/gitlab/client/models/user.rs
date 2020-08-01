use crate::gitlab::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
}
