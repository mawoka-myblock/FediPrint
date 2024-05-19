use std::sync::Arc;
use axum::body::Body;
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::AppState;
use crate::helpers::AppResult;

#[debug_handler]
pub async fn handler(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let data = json!({
        "links": [{
                "rel": "http://nodeinfo.diaspora.software/ns/schema/2.0",
                "href": format!("{}/api/v1/nodeinfo/2.0", state.env.public_url)
            }]
        });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap())
}