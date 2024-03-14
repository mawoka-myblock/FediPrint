use axum::extract::State;
use axum::response::IntoResponse;
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
use crate::models::db::account::{CreateAccount, FullAccount};
use crate::models::db::profile::{CreateProfile, FullProfile};
use crate::{
    helpers::auth::{generate_jwt, get_password_hash},
    helpers::{
        auth::{check_password_hash, InputClaims},
        AppResult,
    },
    models::users::CreateUserInput,
    AppState,
};

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUserInput>,
) -> AppResult<impl IntoResponse> {
    let pw_hash = get_password_hash(input.password);
    let rsa = Rsa::generate(2048).unwrap();
    let public_key = str::from_utf8(&rsa.public_key_to_pem().unwrap())
        .unwrap()
        .to_string();
    let private_key = str::from_utf8(&rsa.private_key_to_pem().unwrap())
        .unwrap()
        .to_string();
    let profile = CreateProfile {
        id: Uuid::now_v7(),
        username: input.username.clone(),
        server: state.env.base_domain.to_string(),
        server_id: format!("{}/api/v1/user/{}", state.env.public_url, input.username),
        display_name: input.display_name,
        inbox: format!(
            "{}/api/v1/user/{}/inbox",
            state.env.public_url, input.username
        ),
        outbox: format!(
            "{}/api/v1/user/{}/outbox",
            state.env.public_url, input.username
        ),
        summary: "".to_string(),
        public_key,
    }
    .create(state.pool.clone())
    .await?;
    CreateAccount {
        password: &pw_hash,
        email: &input.email,
        private_key: &private_key,
        profile_id: &profile.id,
    }
    .create(state.pool.clone())
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
    let acct = match FullAccount::get_by_email(&data.email, state.pool.clone()).await {
        Ok(d) => d,
        Err(_) => return Ok((jar, StatusCode::UNAUTHORIZED)),
    };
    let prof: FullProfile = match FullProfile::get_by_id(&acct.profile_id, state.pool.clone()).await {
        Ok(d) => d,
        Err(_) => return Ok((jar, StatusCode::UNAUTHORIZED)),
    };
    if !check_password_hash(data.password, &acct.password) {
        return Ok((jar, StatusCode::UNAUTHORIZED));
    }
    let rsa_key = Rsa::private_key_from_pem(acct.private_key.as_ref()).unwrap();
    let pkey = PKey::from_rsa(rsa_key).unwrap();
    let encrypted_key = pkey
        .private_key_to_pem_pkcs8_passphrase(Cipher::aes_128_cbc(), state.env.jwt_secret.as_ref())
        .unwrap();
    let claims = InputClaims {
        sub: acct.id,
        profile_id: prof.id,
        display_name: prof.display_name,
        email: acct.email,
        username: prof.username,
        server_id: prof.server_id,
        private_key: general_purpose::STANDARD.encode(encrypted_key),
    };
    let jwt = generate_jwt(claims, state.env.jwt_secret.clone());
    let auth_cookie = Cookie::build(("authorization_key", jwt))
        .secure(true)
        .path("/")
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
