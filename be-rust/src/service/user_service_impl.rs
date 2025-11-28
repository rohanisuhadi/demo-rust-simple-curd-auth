use anyhow::{Result, anyhow};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use axum::async_trait;
use std::result::Result::Ok;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::user::User,
    dto::{
        filter_query::{FilterOperator, FilterParam, FilterValue},
        pagination::{PaginatedResponse, PaginationParams},
        user_pag_req::UserPagReq,
        user_req::UserReq,
    },
    repository::user_repo::UserRepository,
    service::user_service::UserService,
};

pub struct UserServiceImpl<R>
where
    R: UserRepository,
{
    repo: Arc<R>,
}

impl<R> UserServiceImpl<R>
where
    R: UserRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> UserService<R> for UserServiceImpl<R>
where
    R: UserRepository,
{
    async fn get_by_id(&self, id: Uuid) -> Result<User> {
        let user: User = self.repo.find_by_id(id).await.map_err(|e| {
            println!("DB error: {:?}", e);
            anyhow!("ID not found")
        })?;

        Ok(user)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool> {
        let status: bool = self.repo.delete_by_id(id).await.map_err(|e| {
            println!("DB error: {:?}", e);
            anyhow!("ID not found")
        })?;

        Ok(status)
    }

    async fn update_by_id(&self, user: User) -> Result<User> {
        let user: User = self.repo.update_by_id(&user).await.map_err(|e| {
            println!("DB error: {:?}", e);
            anyhow!("ID not found")
        })?;
        Ok(user)
    }

    fn hash_password_new(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| anyhow::anyhow!("Failed to hash password"))?
            .to_string();

        Ok(password_hash)
    }

    async fn save(&self, mut user: UserReq) -> Result<User> {
        if let Some(password) = user.password.take() {
            let hashed = Self::hash_password_new(&password)?;
            user.password = Some(hashed);
        }

        user.id = Some(Uuid::new_v4());
        let user: User = self.repo.save(&user).await.map_err(|e| {
            println!("DB error: {:?}", e);
            anyhow!("ID not found")
        })?;
        Ok(user)
    }

    async fn get_paginated_users(
        &self,
        req: UserPagReq,
    ) -> Result<PaginatedResponse<User>, anyhow::Error> {
        let mut filter_params = vec![];

        if req.email.is_some() {
            let filter_email = FilterParam {
                column: "email".to_string(),
                op: FilterOperator::Eq,
                value: FilterValue::String(req.email.unwrap()),
            };
            filter_params.push(filter_email);
        }

        if req.active.is_some() {
            let filter_active = FilterParam {
                column: "active".to_string(),
                op: FilterOperator::Eq,
                value: FilterValue::Bool(req.active.unwrap()),
            };
            filter_params.push(filter_active);
        }

        if req.name.is_some() {
            let filter_active = FilterParam {
                column: "full_name".to_string(),
                op: FilterOperator::Like,
                value: FilterValue::Like(req.name.unwrap()),
            };
            filter_params.push(filter_active);
        }

        let pagination = PaginationParams {
            page: req.page.unwrap_or_else(|| 1),
            per_page: req.per_page.unwrap_or_else(|| 15),
        };

        let res = self.repo.paginate(filter_params, &pagination).await?;
        Ok(res)
    }
}
