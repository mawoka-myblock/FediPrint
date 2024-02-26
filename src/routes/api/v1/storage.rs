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
    // Stolen from here: https://users.rust-lang.org/t/upload-and-download-with-axum-streaming/85831
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        match put_file(&state.s3, &filename, body_reader).await {
            Ok(_) => (),
            Err(e) => {
                println!("{:?}", e);
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(""))
                    .unwrap());
            }
        };

        return Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::from(""))
            .unwrap());
    }

    return Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(""))
        .unwrap());
}