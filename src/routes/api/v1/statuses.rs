use std::sync::Arc;
use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use crate::AppState;
use crate::helpers::{AppResult, ensure_ap_header};
use crate::models::activitypub::ActivityPubModel;
use crate::routes::api::v1::model::GetModelQuery;

#[debug_handler]
pub async fn get_status(
    State(state): State<Arc<AppState>>,
    query: Query<GetModelQuery>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };
    let model = ActivityPubModel::get_by_id(&query.id, state.pool.clone(), state.env.public_url.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&model).unwrap()))
        .unwrap())
}