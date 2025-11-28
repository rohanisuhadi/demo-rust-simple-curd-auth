use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    app_state::AppState,
    dto::jwt_key::JwtKeys,
    http::routes::create_routes,
    repository::user_repo_impl::UserRepositoryImpl,
    service::{auth_service_impl::AuthServiceImpl, user_service_impl::UserServiceImpl},
};

mod app_state;
mod domain;
mod dto;
mod http;
mod middleware;
mod repository;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let repo = Arc::new(UserRepositoryImpl { pool });

    let user = UserServiceImpl::new(Arc::clone(&repo));
    let jwt_key = JwtKeys::from_files("key/private.pem", "key/public.pem");
    let auth = AuthServiceImpl::new(Arc::clone(&repo), Arc::new(jwt_key));

    let state = AppState::new(auth, user);
    let app = create_routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on http://0.0.0.0:8000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
