use crate::helpers::auth::UserState;
use crate::helpers::{AppResult, internal_app_error};
use crate::models::model::CreateModel;
use crate::models::db::model::{CreateModel as DbCreateModel, FullModel};
use crate::AppState;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde_derive::Deserialize;
use std::sync::Arc;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use crate::routes::api::v1::storage::PaginationQuery;
use diesel_async::{RunQueryDsl};
use uuid::Uuid;

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
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    use crate::schema::Model::table;
    let res = diesel::insert_into(table)
        .values(&DbCreateModel {
            server: state.env.base_domain.to_string(),
            server_id: None,
            profile_id: claims.profile_id,
            published: false,
            title: input.title,
            summary: input.summary,
            description: input.description,
            tags: Some(input.tags),
        }).returning(FullModel::as_returning()).get_result(&mut conn).await?;
    let s_id = format!(
        "{}/api/v1/models/{}/{}",
        state.env.public_url, claims.username, &res.id
    );
    use crate::schema::Model::dsl::*;
    let model = diesel::update(Model.find(res.id)).set(server_id.eq(s_id)).returning(FullModel::as_returning()).get_result(&mut conn).await?;
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
    use crate::schema::Model::dsl::*;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let models = Model.filter(profile_id.eq(claims.profile_id)).select(FullModel::as_select()).load(&mut conn).await?;
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
    use crate::schema::Model::dsl::*;
    use crate::schema::Model::table;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let model = diesel::update(table)
        .filter(id.eq(input.model_id))
        .filter(profile_id.eq(claims.profile_id))
        .set(published.eq(input.public))
        .returning(FullModel::as_returning())
        .get_result(&mut conn)
        .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&model).unwrap()))
        .unwrap())
}

#[debug_handler]
pub async fn get_newest_models(State(state): State<Arc<AppState>>, query: Query<PaginationQuery>) -> AppResult<impl IntoResponse> {
    if query.page < 0 {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("page can't be less than 0"))
            .unwrap());
    }
    use crate::schema::Model::dsl::*;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let models = Model.filter(published.eq(true)).order(created_at.asc()).offset((&query.page * 20) as i64).limit(20).select(FullModel::as_select()).load(&mut conn).await?;
    return Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&models).unwrap()))
        .unwrap());
}