use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::models::db::printer::CreatePrinter as DbCreatePrinter;
use crate::models::db::printer::FullPrinter;
use crate::models::printers::{CreatePrinter, UpdatePrinter};
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
    let printer_data = DbCreatePrinter {
        name: input.name,
        manufacturer: input.manufacturer,
        profile_id: claims.profile_id,
        public: input.public,
        slicer_config: input.slicer_config,
        slicer_config_public: input.slicer_config_public,
        description: input.description,
        modified_scale: input.modified_scale,
    }
    .create(state.pool.clone())
    .await?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&printer_data).unwrap()))
        .unwrap())
}

pub async fn get_all_printers(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let printers =
        FullPrinter::get_all_printer_by_profile(&claims.profile_id, state.pool.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
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
    let printer_data = DbCreatePrinter {
        name: input.name,
        manufacturer: input.manufacturer,
        profile_id: claims.profile_id,
        public: input.public,
        slicer_config: input.slicer_config,
        slicer_config_public: input.slicer_config_public,
        description: input.description,
        modified_scale: input.modified_scale,
    }
    .update_by_id_and_profile_id(&input.id, &claims.profile_id, state.pool.clone())
    .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&printer_data).unwrap()))
        .unwrap())
}
