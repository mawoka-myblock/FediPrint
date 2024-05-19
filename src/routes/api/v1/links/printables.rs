use crate::helpers::auth::UserState;
use crate::helpers::printables::{
    check_printables_profile, import_all_models, import_single_model, CheckPrintablesProfile,
    ImportModelResponse,
};
use crate::helpers::AppResult;
use database::::db::model::FullModel;
use database::::db::profile::FullProfile;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde_derive::Deserialize;
use std::sync::Arc;
use tracing::debug;

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

#[debug_handler]
pub async fn import_all_from_printables(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let profile = FullProfile::get_by_id(&claims.profile_id, state.pool.clone()).await?;
    if profile.linked_printables_profile.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::PRECONDITION_FAILED)
            .body(Body::from("Printables Account not linked"))
            .unwrap());
    }
    match import_all_models(profile, state).await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(""))
            .unwrap()),
        Err(e) => {
            debug!("{:?}", e);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(""))
                .unwrap())
        }
    }
}

#[derive(Deserialize)]
pub struct ImportSingleModelFromPrintablesInput {
    pub id: i64,
}

#[debug_handler]
pub async fn import_one_from_printables(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<ImportSingleModelFromPrintablesInput>,
) -> AppResult<impl IntoResponse> {
    let profile = FullProfile::get_by_id(&claims.profile_id, state.pool.clone()).await?;
    if profile.linked_printables_profile.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::PRECONDITION_FAILED)
            .body(Body::from("Printables Account not linked"))
            .unwrap());
    }
    let mut resp_code = StatusCode::OK;
    let model: Option<FullModel> =
        match import_single_model(&input.id.to_string(), profile, state).await {
            Ok(d) => Some(d),
            Err(e) => {
                resp_code = match e {
                    ImportModelResponse::ModelNotFound => StatusCode::NOT_FOUND,
                    ImportModelResponse::OtherError => StatusCode::INTERNAL_SERVER_ERROR,
                    ImportModelResponse::RequestError => StatusCode::INTERNAL_SERVER_ERROR,
                    ImportModelResponse::NotModelAuthor => StatusCode::UNAUTHORIZED,
                };
                None
            }
        };
    match model {
        Some(d) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string(&d).unwrap()))
            .unwrap()),
        None => Ok(Response::builder()
            .status(resp_code)
            .body(Body::from(""))
            .unwrap()),
    }
}
