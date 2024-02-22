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
use uuid::Uuid;

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
        let private_key = PKey::private_key_from_pkcs8_passphrase(&pkcs8_key, key.as_ref())?;
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

pub fn check_if_token_was_valid(jwt: String, secret: String) -> Result<Claims, ()> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let jwt_data = match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Err(_) => return Err(()),
        Ok(d) => d,
    };
    let expiration_timestamp = jwt_data.claims.exp;
    let six_months_ago = Utc::now() - Duration::days(30);
    let exp_ts: i64 = match expiration_timestamp.try_into() {
        Ok(d) => d,
        Err(_) => return Err(()),
    };
    return if exp_ts >= six_months_ago.timestamp() {
        Ok(jwt_data.claims)
    } else {
        Err(())
    };
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
