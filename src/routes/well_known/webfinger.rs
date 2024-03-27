use crate::helpers::AppResult;
use crate::models::data::{Webfinger, WebfingerLink};
use crate::models::db::profile::FullProfile;
use crate::AppState;
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
                .unwrap());
        }
    };
    let username = match caps.get(1) {
        Some(d) => d.as_str(),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(""))
                .unwrap());
        }
    };
    let domain = match caps.get(2) {
        Some(d) => d.as_str(),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(""))
                .unwrap());
        }
    };
    if domain != state.env.base_domain {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(""))
            .unwrap());
    }

    let user = FullProfile::get_by_username_and_server(
        username,
        &state.env.base_domain,
        state.pool.clone(),
    )
    .await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_state;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use std::clone::Clone;

    #[sqlx::test(fixtures("../api/v1/fixtures/basic_user.sql"))]
    async fn test_webfinger_handler(pool: PgPool) {
        let state = State(get_state(Some(pool.clone())).await);
        // let ext: Extension<UserState> = Extension(UserState::get_fake(pool.clone()).await);
        // Get simple user
        let res = handler(
            state.clone(),
            Query(WebfingerQuery {
                resource: "acct:testuser@localhost.local".to_string(),
            }),
        )
        .await
        .into_response();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            res.headers().get("Content-Type").unwrap(),
            "application/jrd+json; charset=utf-8"
        );
        let j: Webfinger =
            serde_json::from_slice(&*res.into_body().collect().await.unwrap().to_bytes()).unwrap();
        assert_eq!(j.subject, "acct:testuser@localhost.local");
        // Check correctness of server
        let res = handler(
            state.clone(),
            Query(WebfingerQuery {
                resource: "acct:testuser@mastodon.online".to_string(),
            }),
        )
        .await
        .into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        // Check existance of username
        let res = handler(
            state.clone(),
            Query(WebfingerQuery {
                resource: "acct:@mastodon.online".to_string(),
            }),
        )
        .await
        .into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let res = handler(
            state.clone(),
            Query(WebfingerQuery {
                resource: "acct:mastodon.online".to_string(),
            }),
        )
        .await
        .into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        // Check existance of Server
        let res = handler(
            state.clone(),
            Query(WebfingerQuery {
                resource: "acct:testuser@".to_string(),
            }),
        )
        .await
        .into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        let res = handler(
            state.clone(),
            Query(WebfingerQuery {
                resource: "acct:testuser".to_string(),
            }),
        )
        .await
        .into_response();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }
}
