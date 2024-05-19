use crate::helpers::auth::UserState;
use crate::helpers::interactions::{create_remote_profile, follow_user};
use crate::helpers::AppResult;
use database::::db::profile::FullProfile;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
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
pub async fn follow_user_route(
    Extension(claims): Extension<UserState>,
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
    let to_follow: FullProfile =
        match FullProfile::get_by_username_and_server(username, domain, state.pool.clone()).await {
            Ok(d) => d,
            Err(_) => {
                match create_remote_profile(
                    username.to_string(),
                    domain.to_string(),
                    state.pool.clone(),
                )
                .await
                {
                    Ok(d) => d,
                    Err(e) => {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(e.to_string()))
                            .unwrap());
                    }
                }
            }
        };
    match follow_user(&to_follow, &claims, &state.env).await {
        Ok(_) => (),
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(e.to_string()))
                .unwrap());
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(""))
        .unwrap())
}
