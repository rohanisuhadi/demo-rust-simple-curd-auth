use axum::body::Body;
use http::{Request, Response, StatusCode};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json::json;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

use crate::dto::token_claim::TokenClaims;

#[derive(Clone)]
pub struct RequireAuthLayer {
    jwt_secret: String,
}

impl RequireAuthLayer {
    // pub fn new(secret: impl Into<String>) -> Self {
    //     Self {
    //         jwt_secret: secret.into(),
    //     }
    // }

    // pub fn from_env() -> Self {
    //     let secret =
    //         std::env::var("JWT_SECRET").expect("ERROR: JWT_SECRET env variable must be set!");

    //     Self { jwt_secret: secret }
    // }
}

impl<S> Layer<S> for RequireAuthLayer {
    type Service = RequireAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequireAuthMiddleware {
            inner,
            jwt_secret: self.jwt_secret.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RequireAuthMiddleware<S> {
    inner: S,
    jwt_secret: String,
}

impl<S, B> Service<Request<B>> for RequireAuthMiddleware<S>
where
    S: Service<Request<B>, Response = Response<Body>> + Clone + Send + 'static,
    S::Error: Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let secret = self.jwt_secret.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let header = req.headers().get("Authorization");

            println!("header {:?}", header);

            let auth = match header.and_then(|h| h.to_str().ok()) {
                Some(v) if v.starts_with("Bearer ") => v.trim_start_matches("Bearer ").to_string(),
                _ => {
                    return Ok(json_error(
                        StatusCode::UNAUTHORIZED,
                        "Missing or invalid token",
                    ));
                }
            };

            let decoded = decode::<TokenClaims>(
                &auth,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            );

            let claims = match decoded {
                Ok(c) => c.claims,
                Err(_) => {
                    return Ok(json_error(
                        StatusCode::UNAUTHORIZED,
                        "Invalid or expired token",
                    ));
                }
            };

            // insert claims
            let mut req = req;
            req.extensions_mut().insert(claims);

            inner.call(req).await
        })
    }
}

fn json_error(status: StatusCode, message: &str) -> Response<Body> {
    let payload = json!({ "error": message }).to_string();
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap()
}
