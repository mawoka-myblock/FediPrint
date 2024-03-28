use crate::helpers::auth::UserState;
use crate::helpers::printables::{check_printables_profile, CheckPrintablesProfile};
use crate::helpers::AppResult;
use crate::models::db::profile::FullProfile;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde_derive::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LinkToPrintablesInput {
    pub printables_username: String,
}

#[debug_handler]
pub async fn link_to_printables(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<LinkToPrintablesInput>,
) -> AppResult<impl IntoResponse> {
    let profile = FullProfile::get_by_id(&claims.profile_id, state.pool.clone()).await?;
    if profile.linked_printables_profile.is_some() {
        return Ok(Response::builder()
            .status(StatusCode::CONFLICT)
            .body(Body::from("Account already linked"))
            .unwrap());
    }
    // let printables_resp = check_printables_profile(&input.printables_username, &profile.id, &state.env.public_url).await?;
    let printables_resp =
        check_printables_profile(&input.printables_username, &profile.id, "https://mawoka.eu")
            .await?;
    if printables_resp == CheckPrintablesProfile::UserNotFound {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("printables_profile not found"))
            .unwrap());
    };
    if printables_resp == CheckPrintablesProfile::LinkNotFound {
        return Ok(Response::builder()
            .status(StatusCode::PRECONDITION_FAILED)
            .body(Body::from("Link on profile not found"))
            .unwrap());
    }
    if printables_resp == CheckPrintablesProfile::IsOk {
        let profile = profile
            .link_printables_profile(&input.printables_username, state.pool.clone())
            .await?;
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string(&profile).unwrap()))
            .unwrap());
    }
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(""))
        .unwrap())
}
