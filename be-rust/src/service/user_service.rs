use anyhow::Result;
use axum::async_trait;
use uuid::Uuid;

use crate::{
    domain::user::User,
    dto::{pagination::PaginatedResponse, user_pag_req::UserPagReq, user_req::UserReq},
    repository::user_repo::UserRepository,
};

#[async_trait]
pub trait UserService<R>: Send + Sync
where
    R: UserRepository,
{
    async fn get_by_id(&self, id: Uuid) -> Result<User>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool>;
    async fn update_by_id(&self, user: User) -> Result<User>;
    async fn save(&self, user: UserReq) -> Result<User>;
    async fn get_paginated_users(
        &self,
        req: UserPagReq,
    ) -> Result<PaginatedResponse<User>, anyhow::Error>;
    fn hash_password_new(password: &str) -> Result<String>;
}
