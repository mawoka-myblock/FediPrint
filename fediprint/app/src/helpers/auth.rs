use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Local, Utc};
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use openssl::pkey::{PKey, Private};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[cfg(test)]
use crate::{TEST_ACCOUNT_UUID, TEST_PROFILE_UUID};
#[cfg(test)]
use openssl::rsa::Rsa;
#[cfg(test)]
use shared::db::account::FullAccount;
#[cfg(test)]
use shared::db::profile::FullProfile;
#[cfg(test)]
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub profile_id: Uuid,
    pub server_id: String,
    pub private_key: String,
    pub exp: i64,
    pub iat: i64,
}

// Same as claims, but excluding exp and iat
pub struct InputClaims {
    pub sub: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub profile_id: Uuid,
    pub server_id: String,
    pub private_key: String,
}

#[derive(Debug, Clone)]
pub struct UserState {
    pub sub: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub profile_id: Uuid,
    pub server_id: String,
    pub exp: i64,
    pub iat: i64,
    pub private_key: PKey<Private>,
}

impl UserState {
    pub fn from_claims(input: Claims, key: &str) -> anyhow::Result<UserState> {
        use base64::{engine::general_purpose, Engine as _};
        let pkcs8_key = general_purpose::STANDARD.decode(input.private_key)?;
        let private_key = PKey::private_key_from_pem_passphrase(&pkcs8_key, key.as_ref())?;
        Ok(UserState {
            sub: input.sub,
            email: input.email,
            username: input.username,
            display_name: input.display_name,
            profile_id: input.profile_id,
            server_id: input.server_id,
            iat: input.iat,
            exp: input.exp,
            private_key,
        })
    }
    #[cfg(test)]
    pub async fn get_fake(pool: PgPool) -> UserState {
        let account = FullAccount::get_by_id(&TEST_ACCOUNT_UUID, pool.clone())
            .await
            .unwrap();
        let profile = FullProfile::get_by_id(&TEST_PROFILE_UUID, pool.clone())
            .await
            .unwrap();
        let rsa_key = Rsa::private_key_from_pem(account.private_key.as_ref()).unwrap();
        let pkey = PKey::from_rsa(rsa_key).unwrap();
        UserState {
            sub: account.id,
            email: account.email,
            username: profile.username,
            display_name: profile.display_name,
            profile_id: profile.id,
            server_id: profile.server_id,
            iat: 123456i64,
            exp: 123456i64,
            private_key: pkey,
        }
    }
}

pub fn generate_jwt(input_claims: InputClaims, secret: String) -> String {
    let now = Local::now();
    let claims = Claims {
        display_name: input_claims.display_name,
        email: input_claims.email,
        profile_id: input_claims.profile_id,
        username: input_claims.username,
        server_id: input_claims.server_id,
        private_key: input_claims.private_key,
        sub: input_claims.sub,
        iat: now.timestamp(),
        exp: (now + Duration::hours(1)).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn read_jwt(jwt: String, secret: String) -> Result<TokenData<Claims>, errors::Error> {
    decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}

#[derive(Debug)]
pub struct FailedToCheckToken;

impl fmt::Display for FailedToCheckToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FailedToCheckToken")
    }
}

impl std::error::Error for FailedToCheckToken {}

pub fn check_if_token_was_valid(jwt: String, secret: String) -> Result<Claims, FailedToCheckToken> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let jwt_data = match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Err(_) => return Err(FailedToCheckToken),
        Ok(d) => d,
    };
    let expiration_timestamp = jwt_data.claims.exp;
    let six_months_ago = Utc::now() - Duration::days(30);
    let exp_ts: i64 = expiration_timestamp;
    if exp_ts >= six_months_ago.timestamp() {
        Ok(jwt_data.claims)
    } else {
        Err(FailedToCheckToken)
    }
}

pub fn get_password_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn check_password_hash(plain_password: String, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password).unwrap();
    Argon2::default()
        .verify_password(plain_password.as_bytes(), &parsed_hash)
        .is_ok()
}
