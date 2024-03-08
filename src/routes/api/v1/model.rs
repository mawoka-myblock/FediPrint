use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::models::model::CreateModel;
use crate::prisma::{file, model, profile};
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use std::sync::Arc;

#[debug_handler]
pub async fn create_model(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateModel>,
) -> AppResult<impl IntoResponse> {
    let mut images_vec: Vec<file::UniqueWhereParam> = vec![];
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
        "{}/api/v1/notes/{}/{}",
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
