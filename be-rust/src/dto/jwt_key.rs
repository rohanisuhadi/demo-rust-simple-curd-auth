use jsonwebtoken::{DecodingKey, EncodingKey};
use std::fs;

pub struct JwtKeys {
    pub enc: EncodingKey,
    pub dec: DecodingKey,
}

impl JwtKeys {
    pub fn from_files(private: &str, public: &str) -> Self {
        let priv_key = fs::read(private).expect("read private.pem");
        let pub_key = fs::read(public).expect("read public.pem");

        Self {
            enc: EncodingKey::from_rsa_pem(&priv_key).expect("invalid private key"),
            dec: DecodingKey::from_rsa_pem(&pub_key).expect("invalid public key"),
        }
    }
}
