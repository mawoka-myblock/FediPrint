use crate::helpers::{ensure_ap_header, AppResult};
use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use shared::helpers::activities::{get_remote_activity, ModelOrNote};
use shared::models::activitypub::ActivityPubModel;
use shared::AppState;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

#[debug_handler]
pub async fn get_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    debug!("Working...");
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };
    let model =
        ActivityPubModel::get_by_id(&id, state.pool.clone(), state.env.public_url.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&model).unwrap()))
        .unwrap())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRemoteStatusQuery {
    pub link: String,
}

#[debug_handler]
pub async fn get_remote_status(
    Query(data): Query<GetRemoteStatusQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let res = get_remote_activity(data.link, state.clone()).await;
    match res {
        Ok(d) => {
            let body = match d {
                ModelOrNote::Model(d) => serde_json::to_string(&d).unwrap(),
                ModelOrNote::Note(d) => serde_json::to_string(&d).unwrap(),
            };
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(body))
                .unwrap())
        }
        Err(e) => {
            // Handle the error case
            error!("Error while fetching remote thing: {:?}", e);
            Ok(StatusCode::NOT_ACCEPTABLE.into_response())
        }
    }
}
