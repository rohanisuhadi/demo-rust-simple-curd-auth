use axum::async_trait;
use uuid::Uuid;

use crate::domain::user::User;
use crate::dto::filter_query::FilterParam;
use crate::dto::pagination::{PaginatedResponse, PaginationParams};
use crate::dto::user_req::UserReq;
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<User, sqlx::Error>;
    async fn update_by_id(&self, user: &User) -> Result<User, sqlx::Error>;
    async fn save(&self, user: &UserReq) -> Result<User, sqlx::Error>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, sqlx::Error>;
    async fn paginate(
        &self,
        filter_params: Vec<FilterParam>,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<User>, sqlx::Error>;
}
