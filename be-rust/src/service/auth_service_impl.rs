use std::sync::Arc;

use anyhow::{Ok, anyhow};

use argon2::{Argon2, PasswordHash, PasswordVerifier};

use axum::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};
use uuid::Uuid;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    domain::user::User,
    dto::{jwt_key::JwtKeys, login_res::LoginRes, token_claim::TokenClaims},
    repository::user_repo::UserRepository,
    service::auth_service::AuthService,
};

pub struct AuthServiceImpl<R>
where
    R: UserRepository,
{
    repo: Arc<R>,
    jwt_keys: Arc<JwtKeys>,
}

impl<R> AuthServiceImpl<R>
where
    R: UserRepository,
{
    pub fn new(repo: Arc<R>, jwt_keys: Arc<JwtKeys>) -> Self {
        Self { repo, jwt_keys }
    }
}

#[async_trait]
impl<R> AuthService<R> for AuthServiceImpl<R>
where
    R: UserRepository,
{
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<LoginRes> {
        let user: User = self.repo.find_by_email(email).await.map_err(|e| {
            println!("DB error: {:?}", e);
            anyhow!("Email and password wrong")
        })?;

        if user.password.is_none() {
            return Err(anyhow!("Email and password wrong"));
        }

        let password_hash = &user.password.unwrap();
        let parsed =
            PasswordHash::new(password_hash).map_err(|_| anyhow!("Invalid password hash"))?;

        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok();

        if !is_valid {
            return Err(anyhow!("Email and password wrong"));
        }

        // let secret = env::var("JWT_SECRET")?;
        let now = Utc::now();
        let exp = now + Duration::hours(6);

        // let claims = TokenClaims {
        //     sub: user.id,
        //     iat: now.timestamp(),
        //     exp: now.timestamp() + 86400,
        // };

        // let token = encode(
        //     &Header::default(),
        //     &claims,
        //     &EncodingKey::from_secret(secret.as_bytes()),
        // )?;
        //
        let token = self.create_token(user.id);

        Ok(LoginRes {
            id: user.id,
            full_name: user.full_name,
            token: token,
            expired_at: exp,
        })
    }

    fn create_token(&self, user_id: Uuid) -> String {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60 * 60; // 1 jam

        let claims = TokenClaims {
            sub: user_id,
            exp: exp as i64,
            iat: exp as i64,
        };

        let header = Header::new(Algorithm::RS256);
        encode(&header, &claims, &self.jwt_keys.enc).expect("token encode error")
    }

    fn decode_token(&self, token: &str) -> Result<TokenClaims, String> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;

        decode::<TokenClaims>(token, &self.jwt_keys.dec, &validation)
            .map(|data| data.claims)
            .map_err(|e| format!("Token error: {}", e))
    }
}
