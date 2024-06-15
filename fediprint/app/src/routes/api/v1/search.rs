use crate::helpers::AppResult;
use crate::AppState;
use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lazy_static::lazy_static;
use regex::Regex;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use shared::db::profile::FullProfile;
use shared::helpers::instances::get_instance_by_base_url;
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchProfilesQuery {
    pub q: String,
}

lazy_static! {
    static ref HANDLE_REGEX: Regex = Regex::new(r"^@?(?<name>.*)@(?<server>.*\.[a-zA-Z0-9]{1,6})$").unwrap();
    // https://regex101.com/r/37xsch/2
}

#[debug_handler]
pub async fn search_profiles(
    Query(query): Query<SearchProfilesQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let Some(re_res) = HANDLE_REGEX.captures(&query.q) else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };
    let name = &re_res["name"];
    if name.is_empty() {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }
    let server = &re_res["server"];
    if server.is_empty() {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }
    debug!("Parsed User handle: User: {}, Server: {}", &name, &server);
    let instance =
        match get_instance_by_base_url(&format!("https://{server}"), state.pool.clone()).await {
            Ok(d) => d,
            Err(e) => {
                error!("Instance fetch failed: {e}");
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to fetch instance",
                )
                    .into_response());
            }
        };

    let profile = match FullProfile::get_by_name_and_instance_remote(
        name,
        &query.q,
        instance,
        state.pool.clone(),
    )
    .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Profile fetch failed: {e}");
            return Ok(
                (StatusCode::INTERNAL_SERVER_ERROR, "failed to fetch profile").into_response(),
            );
        }
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&profile).unwrap()))
        .unwrap())
}
