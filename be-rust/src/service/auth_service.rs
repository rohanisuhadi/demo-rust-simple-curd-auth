use anyhow::Result;
use axum::async_trait;
use uuid::Uuid;

use crate::{
    dto::{login_res::LoginRes, token_claim::TokenClaims},
    repository::user_repo::UserRepository,
};

#[async_trait]
pub trait AuthService<R>: Send + Sync
where
    R: UserRepository,
{
    async fn login(&self, email: &str, password: &str) -> Result<LoginRes>;
    fn create_token(&self, user_id: Uuid) -> String;
    fn decode_token(&self, token: &str) -> Result<TokenClaims, String>;
}
