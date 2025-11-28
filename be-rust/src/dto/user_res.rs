use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct UserRes {
    pub id: Uuid,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: String,
    pub active: Option<bool>,
    pub image_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

impl UserRes {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            full_name: user.full_name,
            phone_number: user.phone_number,
            email: user.email,
            active: user.active,
            image_url: user.image_url,
            created_at: user.created_at,
        }
    }
}
