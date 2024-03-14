use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::models::db::model::{CreateModel as DbCreateModel, FullModel};
use crate::models::model::CreateModel;
use crate::routes::api::v1::storage::PaginationQuery;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde_derive::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use crate::helpers::search::index_model;

#[debug_handler]
pub async fn create_model(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateModel>,
) -> AppResult<impl IntoResponse> {
    // let mut images_vec: Vec<file::UniqueWhereParam> = vec![];
    if input.images.len() < 1 || input.files.len() < 1 {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Images and/or files are missing"))
            .unwrap());
    }
    /*    for image in input.images {
        images_vec.push(file::id::equals(image.to_string()))
    }
    let mut files_vec: Vec<file::UniqueWhereParam> = vec![];
    for f in input.files {
        files_vec.push(file::id::equals(f.to_string()))
    }*/
    let res = DbCreateModel {
        server: state.env.base_domain.to_string(),
        server_id: None,
        profile_id: claims.profile_id,
        published: false,
        title: input.title,
        summary: input.summary,
        description: input.description,
        tags: input.tags,
    }
    .create(state.pool.clone())
    .await?;
    let s_id = format!(
        "{}/api/v1/models/{}/{}",
        state.env.public_url, claims.username, &res.id
    );
    let model = FullModel::update_server_id_and_return(&res.id, &s_id, state.pool.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&model).unwrap()))
        .unwrap())
}

#[debug_handler]
pub async fn list_own_models(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    let models = FullModel::get_models_of_profile(&claims.profile_id, state.pool.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&models).unwrap()))
        .unwrap())
}

#[derive(Deserialize)]
pub struct ChangeModelVisibilityInput {
    pub model_id: Uuid,
    pub public: bool,
}

#[debug_handler]
pub async fn change_model_visibility(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<ChangeModelVisibilityInput>,
) -> AppResult<impl IntoResponse> {
    let model = FullModel::change_visibility_with_id_and_profile_id(
        &input.public,
        &input.model_id,
        &claims.profile_id,
        state.pool.clone(),
    )
    .await?;
    index_model(&model, &claims.profile_id, &state.ms).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&model).unwrap()))
        .unwrap())
}

#[debug_handler]
pub async fn get_newest_models(
    State(state): State<Arc<AppState>>,
    query: Query<PaginationQuery>,
) -> AppResult<impl IntoResponse> {
    if query.page < 0 {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("page can't be less than 0"))
            .unwrap());
    }
    let models = FullModel::get_newest_published_models_paginated(
        &20i64,
        &((&query.page * 20) as i64),
        state.pool.clone(),
    )
    .await?;
    return Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&models).unwrap()))
        .unwrap());
}
