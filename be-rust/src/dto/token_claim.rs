use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TokenClaims {
    pub sub: Uuid, // user_id
    pub exp: i64,
    pub iat: i64,
}
