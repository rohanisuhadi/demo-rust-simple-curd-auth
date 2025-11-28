use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: String,
    pub active: Option<bool>,
    pub image_url: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
