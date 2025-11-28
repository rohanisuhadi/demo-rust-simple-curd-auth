use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{
    app_state::AppState, repository::user_repo::UserRepository, service::auth_service::AuthService,
};

pub async fn auth_middleware<A, U, R>(
    State(state): State<AppState<A, U>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    A: AuthService<R>,
    R: UserRepository,
{
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    println!("auth {:?}", auth_header);
    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => h.trim_start_matches("Bearer ").to_string(),
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = state
        .auth_service
        .decode_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
