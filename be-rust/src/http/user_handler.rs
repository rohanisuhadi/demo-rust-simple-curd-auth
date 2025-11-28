use axum::extract::Path;
use axum::extract::Query;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::domain::user::User;
use crate::dto::api_res::ApiHttpResponse;
use crate::dto::pagination::PaginatedResponse;
use crate::dto::user_pag_req::UserPagReq;
use crate::dto::user_req::UserReq;
use crate::dto::user_res::UserRes;
use crate::service::user_service::UserService;
use crate::{app_state::AppState, repository::user_repo::UserRepository};

pub async fn get_user_by_id<A, U, R>(
    State(state): State<AppState<A, U>>,
    Path(id): Path<Uuid>,
) -> ApiHttpResponse<UserRes>
where
    U: UserService<R>,
    R: UserRepository,
{
    let result = state.user_service.get_by_id(id).await;
    match result {
        Ok(user) => ApiHttpResponse::ok(UserRes::new(user)),
        Err(e) => ApiHttpResponse::error(StatusCode::UNAUTHORIZED, e.to_string()),
    }
}

pub async fn delete_user_by_id<A, U, R>(
    State(state): State<AppState<A, U>>,
    Path(id): Path<Uuid>,
) -> ApiHttpResponse<bool>
where
    U: UserService<R>,
    R: UserRepository,
{
    let result = state.user_service.delete_by_id(id).await;
    match result {
        Ok(login_res) => ApiHttpResponse::ok(login_res),
        Err(e) => ApiHttpResponse::error(StatusCode::UNAUTHORIZED, e.to_string()),
    }
}

pub async fn update_user_by_id<A, U, R>(
    State(state): State<AppState<A, U>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UserReq>,
) -> ApiHttpResponse<UserRes>
where
    U: UserService<R>,
    R: UserRepository,
{
    let user_opt = state.user_service.get_by_id(id).await;

    if user_opt.is_err() {
        return ApiHttpResponse::error(StatusCode::NOT_FOUND, user_opt.unwrap_err().to_string());
    }

    let user: User = user_opt.unwrap();

    let us = User {
        id: id,
        full_name: req.full_name,
        phone_number: req.phone_number,
        email: req.email,
        active: req.active,
        image_url: user.image_url,
        created_at: user.created_at,
        password: None,
    };
    let result = state.user_service.update_by_id(us).await;
    match result {
        Ok(user) => ApiHttpResponse::ok(UserRes::new(user)),
        Err(e) => ApiHttpResponse::error(StatusCode::EXPECTATION_FAILED, e.to_string()),
    }
}

pub async fn paginate_user<A, U, R>(
    State(state): State<AppState<A, U>>,
    Query(params): Query<UserPagReq>,
) -> ApiHttpResponse<PaginatedResponse<User>>
where
    U: UserService<R>,
    R: UserRepository,
{
    dbg!(&params);
    let result = state.user_service.get_paginated_users(params).await;
    match result {
        Ok(user) => ApiHttpResponse::ok(user),
        Err(e) => ApiHttpResponse::error(StatusCode::EXPECTATION_FAILED, e.to_string()),
    }
}

pub async fn create_user<A, U, R>(
    State(state): State<AppState<A, U>>,
    Json(req): Json<UserReq>,
) -> ApiHttpResponse<UserRes>
where
    U: UserService<R>,
    R: UserRepository,
{
    let result = state.user_service.save(req).await;
    match result {
        Ok(user) => ApiHttpResponse::ok(UserRes::new(user)),
        Err(e) => ApiHttpResponse::error(StatusCode::EXPECTATION_FAILED, e.to_string()),
    }
}
