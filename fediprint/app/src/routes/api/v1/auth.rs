use axum::body::Body;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension};
use axum::{extract::Json, http::StatusCode, Form};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use email_address::EmailAddress;
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
    AppState,
};
use shared::db::account::{CreateAccount, FullAccount};
use shared::db::profile::{CreateProfile, FullProfile};
use shared::models::users::CreateUserInput;

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUserInput>,
) -> AppResult<impl IntoResponse> {
    if state.env.registration_disabled {
        return Ok(Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body(Body::from("Registration is disabled"))
            .unwrap());
    }
    if !EmailAddress::is_valid(&input.email) {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Email is invalid"))
            .unwrap());
    }
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

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(""))
        .unwrap())
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
    let prof: FullProfile = match FullProfile::get_by_id(&acct.profile_id, state.pool.clone()).await
    {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_state;
    use axum::http::StatusCode;
    use shared::users::CreateUserInput;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        let state = get_state(Some(pool)).await;
        let res = create_user(
            State(state),
            Json(CreateUserInput {
                email: "test@mawoka.eu".to_string(),
                password: "password".to_string(),
                username: "testuser".to_string(),
                display_name: "testuser".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[sqlx::test]
    async fn test_failing_create_user(pool: PgPool) {
        let state = get_state(Some(pool)).await;
        let res = create_user(
            State(state.clone()),
            Json(CreateUserInput {
                email: "testdsads@asdas_".to_string(), // https://github.com/johnstonskj/rust-email_address/issues/23
                password: "password".to_string(),
                username: "testuser".to_string(),
                display_name: "testuser".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert_eq!(res.status(), StatusCode::CREATED);
        let res = create_user(
            State(state.clone()),
            Json(CreateUserInput {
                email: "testdsads".to_string(),
                password: "password".to_string(),
                username: "testuser".to_string(),
                display_name: "testuser".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST)
    }

    #[sqlx::test]
    async fn test_login_user(pool: PgPool) {
        let state = get_state(Some(pool)).await;
        let res = create_user(
            State(state.clone()),
            Json(CreateUserInput {
                email: "test@mawoka.eu".to_string(),
                password: "password".to_string(),
                username: "testuser".to_string(),
                display_name: "testuser".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert_eq!(res.status(), StatusCode::CREATED);
        let cookie_jar = CookieJar::new();
        let res = login(
            State(state.clone()),
            cookie_jar,
            Form(LogIn {
                email: "test@mawoka.eu".to_string(),
                password: "password".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert_eq!(res.status(), StatusCode::OK);
        assert!(res
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("ey"));
        // Test wrong email
        let cookie_jar = CookieJar::new();
        let res = login(
            State(state.clone()),
            cookie_jar,
            Form(LogIn {
                email: "test@mawoka".to_string(),
                password: "password".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert!(res.headers().is_empty());
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        // Test wrong password
        let cookie_jar = CookieJar::new();
        let res = login(
            State(state.clone()),
            cookie_jar,
            Form(LogIn {
                email: "test@mawoka.eu".to_string(),
                password: "passwordFALSE".to_string(),
            }),
        )
        .await
        .unwrap()
        .into_response();
        assert!(res.headers().is_empty());
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
