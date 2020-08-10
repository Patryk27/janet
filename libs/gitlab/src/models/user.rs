use crate::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
}
