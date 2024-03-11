use crate::helpers::auth::UserState;
use crate::helpers::{AppResult, internal_app_error};
use crate::models::printers::{CreatePrinter, UpdatePrinter};
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use std::sync::Arc;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use crate::models::db::printer::CreatePrinter as DbCreatePrinter;
use crate::models::db::printer::FullPrinter;
use diesel_async::RunQueryDsl;

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
    use crate::schema::Printer::table;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let printer_data = diesel::insert_into(table)
        .values(&DbCreatePrinter {
            name: input.name,
            manufacturer: input.manufacturer,
            profile_id: claims.profile_id,
            public: input.public,
            slicer_config: input.slicer_config,
            slicer_config_public: input.slicer_config_public,
            description: input.description,
            modified_scale: input.modified_scale
        })
        .returning(FullPrinter::as_returning())
        .get_result(&mut conn)
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
    use crate::schema::Printer::dsl::*;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let printers = Printer.filter(profile_id.eq(claims.profile_id)).select(FullPrinter::as_select()).load(&mut conn).await?;
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
    use crate::schema::Printer::dsl::*;
    use crate::schema::Printer::table;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let printer_data = diesel::update(table)
        .filter(id.eq(input.id))
        .filter(profile_id.eq(claims.profile_id))
        .set(&DbCreatePrinter {
            name: input.name,
            manufacturer: input.manufacturer,
            profile_id: claims.profile_id,
            public: input.public,
            slicer_config: input.slicer_config,
            slicer_config_public: input.slicer_config_public,
            description: input.description,
            modified_scale: input.modified_scale
        })
        .returning(FullPrinter::as_returning())
        .get_result(&mut conn)
        .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&printer_data).unwrap()))
        .unwrap())
}
