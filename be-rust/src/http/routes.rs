use axum::middleware::from_fn_with_state;
use axum::{Router, routing::delete, routing::get, routing::post, routing::put};

use crate::app_state::AppState;
use crate::http::auth_handler::login_handler;
use crate::http::user_handler::{
    create_user, delete_user_by_id, get_user_by_id, paginate_user, update_user_by_id,
};
use crate::middleware::auth_rs256::auth_middleware;
// use crate::middleware::require_auth_layer::RequireAuthLayer;
use crate::repository::user_repo::UserRepository;
use crate::service::auth_service::AuthService;
use crate::service::user_service::UserService;

pub fn create_routes<A, U, R>(state: AppState<A, U>) -> Router
where
    A: AuthService<R> + Send + Sync + 'static,
    U: UserService<R> + Send + Sync + 'static,
    R: UserRepository + Send + Sync + 'static,
{
    let protected_routes = Router::new()
        .route("/:id", get(get_user_by_id::<A, U, R>))
        .route("/:id", delete(delete_user_by_id::<A, U, R>))
        .route("/:id", put(update_user_by_id::<A, U, R>))
        .route("/all", get(paginate_user::<A, U, R>))
        .route("/", post(create_user::<A, U, R>))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .route("/api/auth/login", post(login_handler::<A, U, R>))
        .nest("/api/user", protected_routes)
        .with_state(state)
}
