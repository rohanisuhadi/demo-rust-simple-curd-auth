use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct LoginRes {
    pub id: Uuid,
    pub full_name: Option<String>,
    pub token: String,
    pub expired_at: DateTime<Utc>,
}
