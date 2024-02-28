use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::models::printers::{CreatePrinter, UpdatePrinter};
use crate::prisma::{printer, profile};
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use std::sync::Arc;

#[debug_handler]
pub async fn create_printer(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreatePrinter>,
) -> AppResult<impl IntoResponse> {
    if (&input.slicer_config).is_some() && input.slicer_config.as_ref().unwrap().len() > 60000 {
        return Ok(Response::builder()
            .status(StatusCode::PAYLOAD_TOO_LARGE)
            .body(Body::from("Slicer Config bigger than 60KB."))
            .unwrap());
    }
    let printer_data = state
        .db
        .printer()
        .create(
            input.name,
            input.manufacturer,
            profile::id::equals(claims.profile_id.to_string()),
            vec![
                printer::slicer_config::set(input.slicer_config),
                printer::slicer_config_public::set(input.slicer_config_public),
                printer::description::set(input.description),
                printer::modified_scale::set(input.modified_scale),
                printer::public::set(input.public),
            ],
        )
        .exec()
        .await?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(serde_json::to_string(&printer_data).unwrap()))
        .unwrap())
}

pub async fn get_all_printers(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let printers = state
        .db
        .printer()
        .find_many(vec![printer::profile_id::equals(
            claims.profile_id.to_string(),
        )])
        .exec()
        .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&printers).unwrap()))
        .unwrap())
}

pub async fn update_printer(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<UpdatePrinter>,
) -> AppResult<impl IntoResponse> {
    if (&input.slicer_config).is_some() && input.slicer_config.as_ref().unwrap().len() > 60000 {
        return Ok(Response::builder()
            .status(StatusCode::PAYLOAD_TOO_LARGE)
            .body(Body::from("Slicer Config bigger than 60KB."))
            .unwrap());
    }
    match state
        .db
        .printer()
        .find_first(vec![
            printer::id::equals(input.id.to_string()),
            printer::profile_id::equals(claims.profile_id.to_string()),
        ])
        .exec()
        .await?
    {
        None => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(""))
                .unwrap())
        }
        Some(_) => (),
    }
    let printer_data = state
        .db
        .printer()
        .update(
            printer::id::equals(input.id.to_string()),
            vec![
                printer::name::set(input.name),
                printer::manufacturer::set(input.manufacturer),
                printer::slicer_config::set(input.slicer_config),
                printer::slicer_config_public::set(input.slicer_config_public),
                printer::description::set(input.description),
                printer::modified_scale::set(input.modified_scale),
                printer::public::set(input.public),
            ],
        )
        .exec()
        .await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&printer_data).unwrap()))
        .unwrap())
}
