use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct NewReminder {
    pub user_id: i64,
    pub project_id: i64,
    pub merge_request_iid: i64,
    pub remind_at: DateTime<Utc>,
}
