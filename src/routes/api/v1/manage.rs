use crate::helpers::auth::Claims;
use crate::helpers::interactions::create_remote_profile;
use crate::helpers::AppResult;
use crate::{prisma, AppState};
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct FollowUser {
    pub user: String,
}

#[debug_handler]
pub async fn follow_user(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<FollowUser>,
) -> AppResult<impl IntoResponse> {
    let user_regex = Regex::new(r"@?(.*)@(.*\..{2,})").unwrap();
    let bad_request = Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(""))
        .unwrap());
    let caps = match user_regex.captures(&input.user) {
        Some(d) => d,
        None => {
            return bad_request;
        }
    };
    let username = match caps.get(1) {
        Some(d) => d.as_str(),
        None => {
            return bad_request;
        }
    };
    let domain = match caps.get(2) {
        Some(d) => d.as_str(),
        None => {
            return bad_request;
        }
    };
    let user = state
        .db
        .profile()
        .find_first(vec![
            prisma::profile::username::equals(username.to_string()),
            prisma::profile::server::equals(domain.to_string()),
        ])
        .exec()
        .await?;
    if user.is_none() {
        match create_remote_profile(username.to_string(), domain.to_string(), &state.db).await {
            Ok(_) => (),
            Err(e) => return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap()),
        };
    }
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(""))
        .unwrap())
}
