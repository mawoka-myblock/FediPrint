use crate::helpers::AppResult;
use crate::models::data::{Webfinger, WebfingerLink};
use crate::{prisma, AppState};
use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct WebfingerQuery {
    pub resource: String,
}

#[debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    query: Query<WebfingerQuery>,
    // ) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
) -> AppResult<impl IntoResponse> {
    let query_regex = Regex::new(r"acct:(.*)@(.*\.\w{2,})").unwrap();
    let caps = match query_regex.captures(&query.resource) {
        Some(d) => d,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(""))
                .unwrap())
        }
    };
    let username = match caps.get(1) {
        Some(d) => d.as_str(),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(""))
                .unwrap())
        }
    };
    let domain = match caps.get(2) {
        Some(d) => d.as_str(),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(""))
                .unwrap())
        }
    };
    if domain != state.env.base_domain {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(""))
            .unwrap());
    }
    let user = match state
        .db
        .profile()
        .find_first(vec![
            prisma::profile::username::equals(username.to_string()),
            prisma::profile::server::equals("local".to_string()),
        ])
        .exec()
        .await?
    {
        Some(d) => d,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap())
        }
    };

    let wf_data = Webfinger {
        subject: format!("acct:{}@{}", user.username, state.env.base_domain),
        aliases: vec![
            format!("{}/api/v1/user/{}", state.env.public_url, user.username),
            format!("{}/@{}", state.env.public_url, user.username),
        ],
        links: vec![
            WebfingerLink {
                rel: "http://webfinger.net/rel/profile-page".to_string(),
                type_field: Some("text/html".to_string()),
                href: Some(format!("{}/@{}", state.env.public_url, user.username)),
                template: None,
            },
            WebfingerLink {
                rel: "self".to_string(),
                type_field: Some("application/activity+json".to_string()),
                href: Some(format!(
                    "{}/api/v1/user/{}",
                    state.env.public_url, user.username
                )),
                template: None,
            },
            WebfingerLink {
                rel: "http://ostatus.org/schema/1.0/subscribe".to_string(),
                type_field: None,
                href: None,
                template: Some(format!(
                    "{}/api/authorize_interaction?uri={{uri}}",
                    state.env.public_url
                )),
            },
        ],
    };

    // Ok((StatusCode::OK, Json(serde_json::to_string(&wf_data))))
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/jrd+json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&wf_data).unwrap()))
        .unwrap())
}
