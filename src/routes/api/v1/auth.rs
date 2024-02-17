use axum::{debug_handler, Extension};
use axum::{extract::Json, http::StatusCode, Form};
use axum_extra::extract::CookieJar;
use openssl::rsa::Rsa;
use serde::Deserialize;
use std::str;
use std::sync::Arc;
use axum::extract::State;
use uuid::Uuid;
use axum_extra::extract::cookie::Cookie;

use crate::{helpers::auth::{generate_jwt, get_password_hash}, helpers::{
    auth::{check_password_hash, InputClaims},
    AppResult,
}, models::users::CreateUserInput, prisma, AppState};
use crate::helpers::auth::Claims;

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
    let profile = state.db
        .profile()
        .create(input.username, input.display_name, public_key, vec![])
        .exec()
        .await?;
    state.db.account()
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
pub async fn login(State(state): State<Arc<AppState>>, jar: CookieJar, Form(data): Form<LogIn>) -> AppResult<(CookieJar, StatusCode)> {
    let user = match state.db
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
    let claims = InputClaims {
        sub: Uuid::parse_str(&user.id).unwrap(),
        profile_id: Uuid::parse_str(&user.profile_id).unwrap(),
        display_name: profile.display_name,
        email: user.email,
        username: profile.username,
    };
    let jwt = generate_jwt(claims);
    let auth_cookie = Cookie::build(("authorization_key", jwt))
        .secure(true)
        .http_only(true);

    Ok((jar.add(auth_cookie), StatusCode::OK))
}

#[debug_handler]
pub async fn get_me_handler(
    Extension(claims): Extension<Claims>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": claims.sub
        })
    });

    Ok((StatusCode::OK, Json(json_response)))
}
