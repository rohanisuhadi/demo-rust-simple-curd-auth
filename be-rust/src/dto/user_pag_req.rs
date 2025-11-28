use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserPagReq {
    pub email: Option<String>,
    pub active: Option<bool>,
    pub name: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
