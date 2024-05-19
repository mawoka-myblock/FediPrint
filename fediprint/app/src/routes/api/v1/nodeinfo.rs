use crate::helpers::AppResult;
use crate::AppState;
use axum::body::Body;
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo<'a> {
    pub version: &'a str,
    pub software: Software<'a>,
    pub protocols: Vec<&'a str>,
    pub services: Value,
    pub usage: Usage,
    pub open_registrations: bool,
    pub metadata: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Software<'a> {
    pub name: &'a str,
    pub version: Option<&'a str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub users: Users,
    pub local_posts: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    pub total: i64,
    pub active_month: i64,
    pub active_halfyear: i64,
}

#[debug_handler]
pub async fn get_nodeinfo(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let ni = NodeInfo {
        version: "2.0",
        software: Software {
            name: "fediprint",
            version: option_env!("CARGO_PKG_VERSION"),
        },
        protocols: vec!["activitypub"],
        usage: Usage {
            local_posts: 123, // TODO
            users: Users {
                active_halfyear: 123,
                active_month: 132,
                total: 123,
            },
        },
        services: json!({"outbound": [], "inbound": []}),
        open_registrations: !state.env.registration_disabled,
        metadata: json!({
            "commit": option_env!("VERGEN_GIT_SHA"),
            "branch": option_env!("VERGEN_GIT_BRANCH"),
            "date": option_env!("VERGEN_GIT_COMMIT_DATE"),
            "commit_count": option_env!("VERGEN_GIT_COMMIT_COUNT")
        }),
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&ni).unwrap()))
        .unwrap())
}
