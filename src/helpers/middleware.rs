use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Response};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
};
use std::sync::Arc;

use crate::AppState;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Serialize;

use crate::helpers::auth::{
    check_if_token_was_valid, generate_jwt, read_jwt, InputClaims, UserState,
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}
pub async fn auth_middleware(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let token = match cookie_jar
        .get("authorization_key")
        .map(|cookie| cookie.value().to_string())
    {
        Some(d) => d,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = read_jwt(token.clone(), data.env.jwt_secret.clone());
    if claims.is_ok() {
        req.extensions_mut()
            .insert(UserState::from_claims(claims.unwrap().claims, &data.env.jwt_secret).unwrap());
        return Ok(next.run(req).await);
    }
    tracing::debug!("Checking if token was valid");
    let invalid_claims = match check_if_token_was_valid(token, data.env.jwt_secret.clone()) {
        Ok(d) => d,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    tracing::debug!("Renewing jwt token");

    let new_jwt = generate_jwt(
        InputClaims {
            sub: invalid_claims.sub.clone(),
            email: invalid_claims.email.clone(),
            profile_id: invalid_claims.profile_id.clone(),
            username: invalid_claims.username.clone(),
            display_name: invalid_claims.display_name.clone(),
            server_id: invalid_claims.server_id.clone(),
            private_key: invalid_claims.private_key.clone(),
        },
        data.env.jwt_secret.clone(),
    );

    let auth_cookie = Cookie::build(("authorization_key", new_jwt))
        .secure(true)
        .http_only(true)
        .build()
        .to_string();
    req.extensions_mut()
        .insert(UserState::from_claims(invalid_claims, &data.env.jwt_secret).unwrap());
    let mut resp = next.run(req).await;
    resp.headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&*auth_cookie).unwrap());

    return Ok(resp);

    /*    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;*/

    /*    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

    let user = user.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;*/
}
