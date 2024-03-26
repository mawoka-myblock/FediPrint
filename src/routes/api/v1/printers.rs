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
    if input.slicer_config.is_some() && input.slicer_config.as_ref().unwrap().len() > 60000 {
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
    if input.slicer_config.is_some() && input.slicer_config.as_ref().unwrap().len() > 60000 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_state;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use crate::models::db::ModifiedScale;

    #[sqlx::test(fixtures("basic_user"))]
    async fn test_create_printer(pool: PgPool) {
        let state = get_state(Some(pool.clone())).await;
        let ext: Extension<UserState> = Extension(UserState::get_fake(pool.clone()).await);
        let printer = CreatePrinter {
            name: "Printer".to_string(),
            manufacturer: "Manufacturer".to_string(),
            slicer_config: Some("Slicer_Config".to_string()),
            description: Some("Description".to_string()),
            modified_scale: ModifiedScale::HardMods,
            public: true,
            slicer_config_public: true,
        };
        let res = create_printer(ext.clone(), State(state.clone()), Json(printer.clone())).await.into_response();
        assert_eq!(res.status(), StatusCode::CREATED);
        // Test duplicate
        let res = create_printer(ext.clone(), State(state.clone()), Json(printer.clone())).await.into_response();
        assert_eq!(res.status(), StatusCode::CONFLICT);
        // Test slicer_config too big
        let mut too_big = printer.clone();
        too_big.slicer_config = Some(std::iter::repeat("x")
            .take(70 * 1000)
            .map(|c| c.parse::<char>().unwrap())
            .collect()
        );
        let res = create_printer(ext.clone(), State(state.clone()), Json(too_big)).await.into_response();
        assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE)
    }

    use std::clone::Clone;
    use serde_json::Value;


    #[sqlx::test(fixtures("basic_user", "private_printers"))]
    async fn test_get_all_printers(pool: PgPool) {
        let state = State(get_state(Some(pool.clone())).await);
        let ext: Extension<UserState> = Extension(UserState::get_fake(pool.clone()).await);
        let res = get_all_printers(ext.clone(), state.clone()).await.into_response();
        assert_eq!(res.status(), StatusCode::OK);
        let b = res.into_body();
        let j: Value = serde_json::from_slice(&*b.collect().await.unwrap().to_bytes()).unwrap();
        let j_s = j.to_string();
        assert!(j_s.contains("Printer") && j_s.contains("Printer2"))
    }
}