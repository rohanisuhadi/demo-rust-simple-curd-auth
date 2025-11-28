use axum::extract::{Json, State};
use axum::http::StatusCode;

use crate::dto::api_res::ApiHttpResponse;
use crate::dto::login_req::LoginReq;
use crate::dto::login_res::LoginRes;
use crate::service::auth_service::AuthService;
use crate::{app_state::AppState, repository::user_repo::UserRepository};

pub async fn login_handler<A, U, R>(
    State(state): State<AppState<A, U>>,
    Json(req): Json<LoginReq>,
) -> ApiHttpResponse<LoginRes>
where
    A: AuthService<R>,
    R: UserRepository,
{
    let result = state.auth_service.login(&req.email, &req.password).await;
    match result {
        Ok(login_res) => ApiHttpResponse::ok(login_res),
        Err(e) => ApiHttpResponse::error(StatusCode::UNAUTHORIZED, e.to_string()),
    }
}
