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
use diesel::RunQueryDsl;
use prisma_client_rust::Direction;
use crate::routes::api::v1::storage::PaginationQuery;
use diesel_async::RunQueryDsl;

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
    let models = state
        .db
        .model()
        .find_many(vec![model::profile_id::equals(
            claims.profile_id.to_string(),
        )])
        .exec()
        .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&models).unwrap()))
        .unwrap())
}

#[derive(Deserialize)]
pub struct ChangeModelVisibilityInput {
    pub model_id: i64,
    pub public: bool,
}

#[debug_handler]
pub async fn change_model_visibility(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<ChangeModelVisibilityInput>,
) -> AppResult<impl IntoResponse> {
    match state
        .db
        .model()
        .find_first(vec![
            model::id::equals(input.model_id),
            model::profile_id::equals(claims.profile_id.to_string()),
        ])
        .exec()
        .await?
    {
        Some(_) => (),
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap());
        }
    }
    let model_data = state
        .db
        .model()
        .update(
            model::id::equals(input.model_id),
            vec![model::published::set(input.public)],
        )
        .exec()
        .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&model_data).unwrap()))
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
    let models = state
        .db
        .model()
        .find_many(vec![
            model::published::equals(true)
        ])
        .order_by(model::created_at::order(Direction::Asc))
        .skip((&query.page * 20) as i64)
        .take(20)
        .exec()
        .await?;
    return Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&models).unwrap()))
        .unwrap());
}