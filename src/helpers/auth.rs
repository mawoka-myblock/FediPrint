use std::env;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Local, Utc};
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub profile_id: Uuid,
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
}


pub fn generate_jwt(input_claims: InputClaims) -> String {
    let now = Local::now();
    let claims = Claims {
        display_name: input_claims.display_name,
        email: input_claims.email,
        profile_id: input_claims.profile_id,
        username: input_claims.username,
        sub: input_claims.sub,
        iat: now.timestamp(),
        exp: (now + Duration::hours(1)).timestamp()

    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

pub fn read_jwt(jwt: String) -> Result<TokenData<Claims>, errors::Error> {
    decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}

pub fn check_if_token_was_valid(jwt: String) -> bool {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let jwt_data = match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
        &validation,
    ) {
        Err(_) => return false,
        Ok(d) => d,
    };
    let expiration_timestamp = jwt_data.claims.exp;
    let six_months_ago = Utc::now() - Duration::days(30);
    let exp_ts: i64 = match expiration_timestamp.try_into() {
        Ok(d) => d,
        Err(_) => return false
    };
    exp_ts >= six_months_ago.timestamp()
}


pub fn get_password_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn check_password_hash(plain_password: String, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password).unwrap();
    Argon2::default().verify_password(plain_password.as_bytes(), &parsed_hash).is_ok()
}