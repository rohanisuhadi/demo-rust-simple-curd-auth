use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct UserReq {
    pub id: Option<Uuid>,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: String,
    pub password: Option<String>,
    pub active: Option<bool>,
}
