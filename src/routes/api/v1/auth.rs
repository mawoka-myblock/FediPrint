use axum::extract::State;
use axum::{debug_handler, Extension};
use axum::{extract::Json, http::StatusCode, Form};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::symm::Cipher;
use serde::Deserialize;
use std::str;
use std::sync::Arc;
use uuid::Uuid;

use crate::helpers::auth::UserState;
use crate::{
    helpers::auth::{generate_jwt, get_password_hash},
    helpers::{
        auth::{check_password_hash, InputClaims},
        AppResult,
    },
    models::users::CreateUserInput,
    prisma, AppState,
};

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUserInput>,
) -> AppResult<StatusCode> {
    let pw_hash = get_password_hash(input.password);
    let rsa = Rsa::generate(2048).unwrap();
    let public_key = str::from_utf8(&rsa.public_key_to_pem().unwrap())
        .unwrap()
        .to_string();
    let private_key = str::from_utf8(&rsa.private_key_to_pem().unwrap())
        .unwrap()
        .to_string();
    // https://github.com/Brendonovich/prisma-client-rust/issues/44
    let profile = state
        .db
        .profile()
        .create(
            input.username.clone(),
            state.env.base_domain.to_string(),
            format!("{}/api/v1/user/{}", state.env.public_url, input.username),
            input.display_name,
            format!(
                "{}/api/v1/user/{}/inbox",
                state.env.public_url, input.username
            ),
            format!(
                "{}/api/v1/user/{}/outbox",
                state.env.public_url, input.username
            ),
            public_key,
            vec![],
        )
        .exec()
        .await?;
    state
        .db
        .account()
        .create(
            pw_hash,
            input.email,
            private_key,
            prisma::profile::id::equals(profile.id),
            vec![],
        )
        .exec()
        .await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct LogIn {
    pub email: String,
    pub password: String,
}

#[debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(data): Form<LogIn>,
) -> AppResult<(CookieJar, StatusCode)> {
    let user = match state
        .db
        .account()
        .find_unique(prisma::account::email::equals(data.email))
        .with(prisma::account::profile::fetch())
        .exec()
        .await?
    {
        None => return Ok((jar, StatusCode::UNAUTHORIZED)),
        Some(d) => d,
    };
    if !check_password_hash(data.password, &user.password) {
        return Ok((jar, StatusCode::UNAUTHORIZED));
    }
    let profile: prisma::profile::Data = match user.profile() {
        Ok(d) => d.clone(),
        Err(_) => return Ok((jar, StatusCode::INTERNAL_SERVER_ERROR)),
    };
    let rsa_key = Rsa::private_key_from_pem(user.private_key.as_ref()).unwrap();
    let pkey = PKey::from_rsa(rsa_key).unwrap();
    let encrypted_key = pkey
        .private_key_to_pem_pkcs8_passphrase(Cipher::aes_128_cbc(), state.env.jwt_secret.as_ref())
        .unwrap();
    let claims = InputClaims {
        sub: Uuid::parse_str(&user.id).unwrap(),
        profile_id: Uuid::parse_str(&user.profile_id).unwrap(),
        display_name: profile.display_name,
        email: user.email,
        username: profile.username,
        server_id: user.profile.unwrap().server_id,
        private_key: general_purpose::STANDARD.encode(encrypted_key),
    };
    let jwt = generate_jwt(claims, state.env.jwt_secret.clone());
    let auth_cookie = Cookie::build(("authorization_key", jwt))
        .secure(true)
        .http_only(true);

    Ok((jar.add(auth_cookie), StatusCode::OK))
}

#[debug_handler]
pub async fn get_me_handler(
    Extension(claims): Extension<UserState>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": claims.sub
        })
    });

    Ok((StatusCode::OK, Json(json_response)))
}
