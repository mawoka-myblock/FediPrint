use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::models::db::file::{CreateFile, FullFile, UpdateFile};
use crate::models::storage::UpdateImageMetadata;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use futures::TryStreamExt;
use s3::Bucket;
use serde_derive::Deserialize;
use std::sync::Arc;
use std::{io, pin::Pin};
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;
use tracing::debug;
use uuid::Uuid;

pub async fn put_file(
    bucket: &Bucket,
    filename: &str,
    content_type: &str,
    mut reader: Pin<&mut (dyn AsyncRead + Send)>,
) -> Result<(), ()> {
    bucket
        .put_object_stream_with_content_type(&mut reader, filename, content_type)
        .await
        .unwrap();
    Ok(())
}

#[debug_handler]
pub async fn upload_file(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    let file_id = Uuid::now_v7();
    let str_id = file_id.to_string();
    let mut filename: String = String::new();
    let mut content_type: String = String::new();
    // Stolen from here: https://users.rust-lang.org/t/upload-and-download-with-axum-streaming/85831
    while let Some(field) = multipart.next_field().await.unwrap() {
        filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };
        content_type = match field.content_type() {
            Some(d) => d.to_string(),
            None => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Content Type not available"))
                    .unwrap());
            }
        };
        debug!("Filename: {}", &filename);
        debug!("Content-Type: {}", &content_type);
        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        match put_file(&state.s3, &str_id, &content_type, body_reader).await {
            Ok(_) => (),
            Err(e) => {
                debug!("{:?}", e);
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(""))
                    .unwrap());
            }
        };
    }
    let s3_metadata = state.s3.head_object(&str_id).await?;
    let file_data = CreateFile {
        id: file_id,
        mime_type: content_type,
        size: s3_metadata.0.content_length.unwrap(),
        profile_id: claims.profile_id,
        file_name: Some(filename),
        preview_file_id: None,
        thumbhash: None,
        description: None,
        alt_text: None,
        file_for_model_id: None,
        image_for_model_id: None,
    }
        .create(state.pool.clone())
        .await?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&file_data).unwrap()))
        .unwrap())
}

#[debug_handler]
pub async fn edit_file_metadata(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<UpdateImageMetadata>,
) -> AppResult<impl IntoResponse> {
    let data = UpdateFile {
        id: input.id,
        thumbhash: input.thumbhash,
        alt_text: input.alt_text,
        file_name: input.file_name,
        description: input.description,
    }
        .update_by_profile_and_return(&claims.profile_id, state.pool.clone())
        .await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap())
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: i16,
}

#[debug_handler]
pub async fn list_own_files(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    query: Query<PaginationQuery>,
) -> AppResult<impl IntoResponse> {
    if query.page < 0 {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("page can't be less than 0"))
            .unwrap());
    }
    let files = FullFile::get_newest_files_by_profile_paginated(
        &claims.profile_id,
        &20i64,
        &((&query.page * 20) as i64),
        state.pool.clone(),
    )
        .await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&files).unwrap()))
        .unwrap())
}

#[derive(Deserialize)]
pub struct DeleteFileQuery {
    pub id: Uuid,
}

#[debug_handler]
pub async fn delete_file(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    query: Query<DeleteFileQuery>,
) -> AppResult<impl IntoResponse> {
    let file =
        FullFile::get_by_id_and_profile_id(&query.id, &claims.profile_id, state.pool.clone())
            .await?;
    let d = state.s3.delete_object(format!("/{}", file.id)).await?;
    debug!("S3 Response: {:?}", d);
    file.delete(state.pool.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(""))
        .unwrap())
}


#[debug_handler]
pub async fn get_file(Path(id): Path<Uuid>, State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let body = Body::from_stream(state.s3.get_object_stream(id.to_string()).await.unwrap().bytes);
    Ok(
        Response::builder()
            .body(body)
            .unwrap()
    )
}