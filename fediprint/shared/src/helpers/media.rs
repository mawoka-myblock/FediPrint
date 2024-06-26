use anyhow::bail;
use chrono::DateTime;
use chrono::Utc;
use futures::TryStreamExt;
use lazy_static::lazy_static;
use reqwest::header::HeaderValue;
use s3::Bucket;
use std::collections::HashMap;
use std::sync::Arc;
use std::{io, pin::Pin};
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;
use tracing::debug;
use uuid::Uuid;

use crate::db::file::FullFile;
use crate::AppState;

fn parse_content_disposition(header: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    // Split the header by ';' to separate the different parts
    for part in header.split(';').map(|s| s.trim()) {
        if let Some((key, value)) = part.split_once('=') {
            // Remove surrounding quotes from the value if present
            let value = value.trim_matches('"');
            params.insert(key.to_string(), value.to_string());
        } else {
            // Handle the case where there is no '=' (e.g., 'attachment')
            params.insert(part.to_string(), String::new());
        }
    }

    params
}

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

lazy_static! {
    static ref EMPTY_HEADER_VALUE: HeaderValue = HeaderValue::from_str("").unwrap();
}

pub async fn handle_single_attachment(
    attachment_url: &str,
    profile_id: Uuid,
    model_id: &Uuid,
    state: Arc<AppState>,
) -> anyhow::Result<FullFile> {
    let res = reqwest::get(attachment_url).await?;
    if res.status() != 200 {
        bail!("Server didn't respond with 200")
    }
    let id = Uuid::now_v7();
    let headers = res.headers().clone();
    let content_type = headers
        .get("Content-Type")
        .unwrap_or(&EMPTY_HEADER_VALUE)
        .to_str()
        .unwrap()
        .to_string();
    let content_length = res.content_length();
    #[allow(clippy::bind_instead_of_map)]
    // if we follow clippy, the compiler complains about unhandled exceptions
    // And this seems fine to me, so just ignoring clippy
    let blurhash = headers
        .get("X-Blurhash")
        .and_then(|v| Some(v.to_str().unwrap().to_string()));
    #[allow(clippy::bind_instead_of_map)]
    let alt_text = headers
        .get("X-Alttext")
        .and_then(|v| Some(v.to_str().unwrap().to_string()));

    let mime_type = headers
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let created_at: Option<DateTime<Utc>> = headers.get("X-Created-At").and_then(|v| {
        v.to_str().ok().and_then(|s| {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .ok()
        })
    });
    let updated_at: Option<DateTime<Utc>> = headers.get("X-Updated-At").and_then(|v| {
        v.to_str().ok().and_then(|s| {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .ok()
        })
    });
    let file_name = headers.get("Content-Disposition").and_then(|v| {
        v.to_str()
            .ok()
            .and_then(|v| parse_content_disposition(v).get("filename").cloned())
    });
    let req_stream = res
        .bytes_stream()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(req_stream);
    futures::pin_mut!(body_reader);

    match put_file(&state.s3, &id.to_string(), &content_type, body_reader).await {
        Ok(_) => (),
        Err(e) => {
            debug!("{:?}", e);
        }
    };
    let mut file_for_model_id: Option<Uuid> = None;
    let mut image_for_model_id: Option<Uuid> = None;
    if content_type.contains("image") {
        image_for_model_id = Some(model_id.to_owned());
    } else {
        file_for_model_id = Some(model_id.to_owned());
    }
    let file = FullFile {
        id,
        alt_text,
        created_at: created_at.unwrap_or(Utc::now()),
        description: None,
        updated_at: updated_at.unwrap_or(Utc::now()),
        file_name,
        mime_type,
        profile_id,
        size: content_length.unwrap_or(0) as i64,
        thumbhash: blurhash,
        file_for_model_id,
        image_for_model_id,
        to_be_deleted_at: None, // TODO caching forever?
        preview_file_id: None,
    };
    file.create_no_return(state.pool.clone()).await?;
    Ok(file)
}

pub async fn handle_media(
    attachment_urls: Vec<&str>,
    model_id: Uuid,
    profile_id: Uuid,
    state: Arc<AppState>,
) -> anyhow::Result<()> {
    for url in attachment_urls {
        handle_single_attachment(url, profile_id, &model_id, state.clone()).await?;
    }
    Ok(())
}
