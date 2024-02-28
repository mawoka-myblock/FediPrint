use std::sync::Arc;
use axum::{debug_handler, Extension};
use axum::body::Body;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::AppState;
use crate::helpers::AppResult;
use crate::helpers::auth::UserState;
use tokio_util::io::StreamReader;
use tokio::io::{AsyncRead};
use std::{io, pin::Pin};
use futures::TryStreamExt;
use s3::Bucket;
use uuid::{uuid, Uuid};
use crate::prisma::{file, profile};


async fn put_file(
    bucket: &Bucket,
    filename: &str,
    mut reader: Pin<&mut (dyn AsyncRead + Send)>,
) -> Result<(), ()> {
    bucket
        .put_object_stream(&mut reader, filename)
        .await
        .unwrap();

    Ok(())
}

#[debug_handler]
pub async fn upload_image(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    let file_id = Uuid::new_v4();
    let str_id = file_id.to_string();
    let mut filename: String = String::new();
    let mut content_type: String = String::new();
    let mut filesize: usize = 0;
    // Stolen from here: https://users.rust-lang.org/t/upload-and-download-with-axum-streaming/85831
    while let Some(field) = multipart.next_field().await.unwrap() {
        filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };
        content_type = match field.content_type() {
            Some(d) => d.to_string(),
            None => return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Content Type not available"))
                .unwrap()),
        };
        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        match put_file(&state.s3, &str_id, body_reader).await {
            Ok(_) => (),
            Err(e) => {
                println!("{:?}", e);
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(""))
                    .unwrap());
            }
        };
    }
    let s3_metadata = state.s3.head_object(&str_id).await?;
    let file_data = state.db.file().create(content_type, s3_metadata.0.content_length.unwrap(), profile::id::equals(claims.profile_id.to_string()), vec![
        file::id::set(str_id),
        file::file_name::set(Some(filename)),
    ]).exec().await?;

    return Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(serde_json::to_string(&file_data).unwrap()))
        .unwrap());
}