use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::models::model::CreateModel;
use crate::prisma::{file, model, profile};
use crate::AppState;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde_derive::Deserialize;
use std::sync::Arc;
use prisma_client_rust::Direction;
use crate::routes::api::v1::storage::PaginationQuery;

#[debug_handler]
pub async fn create_model(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateModel>,
) -> AppResult<impl IntoResponse> {
    let mut images_vec: Vec<file::UniqueWhereParam> = vec![];
    if input.images.len() < 1 || input.files.len() < 1 {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Images and/or files are missing"))
            .unwrap());
    }
    for image in input.images {
        images_vec.push(file::id::equals(image.to_string()))
    }
    let mut files_vec: Vec<file::UniqueWhereParam> = vec![];
    for f in input.files {
        files_vec.push(file::id::equals(f.to_string()))
    }
    let data = state
        .db
        .model()
        .create(
            state.env.base_domain.to_string(),
            profile::id::equals(claims.profile_id.to_string()),
            input.title,
            input.summary,
            input.description,
            vec![
                model::files::connect(files_vec),
                model::images::connect(images_vec),
            ],
        )
        .exec()
        .await?;

    let server_id = format!(
        "{}/api/v1/models/{}/{}",
        state.env.public_url, claims.username, &data.id
    );
    let finished_data = state
        .db
        .model()
        .update(
            model::id::equals(data.id),
            vec![model::server_id::set(Some(server_id))],
        )
        .exec()
        .await?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&finished_data).unwrap()))
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