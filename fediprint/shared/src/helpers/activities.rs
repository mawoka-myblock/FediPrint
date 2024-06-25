use reqwest::StatusCode;
use sqlx::PgPool;
use tracing::error;
use url::Url;

use crate::{
    db::{model::FullModelWithRelationsIds, note::FullNote, profile::FullProfile},
    models::activitypub::note::NoteResponse,
};

use super::instances::get_instance_by_base_url;
#[derive(Debug)]
pub enum ModelOrNote {
    Note(FullNote),
    Model(FullModelWithRelationsIds),
}
#[derive(Debug)]
pub enum GetRemoteActivtyErrors {
    RemoteUrlInvalid,
    CouldNotExtractHost,
    FailedToFetchInstance(String),
    RequestFailed,
    RequestFailedWithCode(StatusCode),
    JsonParsingFailed(String),
    ModelCreationFailed,
    NoteCreationFailed,
    UserQueryFailed,
}

pub async fn get_remote_activity(
    url: String,
    pool: PgPool,
) -> Result<ModelOrNote, GetRemoteActivtyErrors> {
    let parsed_url = match Url::parse(&url) {
        Ok(d) => d,
        Err(_) => return Err(GetRemoteActivtyErrors::RemoteUrlInvalid),
    };
    let instance_host = match parsed_url.host_str() {
        Some(d) => d,
        None => return Err(GetRemoteActivtyErrors::CouldNotExtractHost),
    };
    let instance =
        match get_instance_by_base_url(&format!("https://{instance_host}"), pool.clone()).await {
            Ok(d) => d,
            Err(e) => return Err(GetRemoteActivtyErrors::FailedToFetchInstance(e.to_string())),
        };
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
    );

    let ap_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let req = ap_client.get(url).send().await;
    let res = match req {
        Ok(d) => d,
        Err(_) => return Err(GetRemoteActivtyErrors::RequestFailed),
    };
    if res.status() != 200 {
        return Err(GetRemoteActivtyErrors::RequestFailedWithCode(res.status()));
    };
    let body_text = match res.text().await {
        Ok(d) => d,
        Err(e) => {
            error!(e = %e);
            return Err(GetRemoteActivtyErrors::JsonParsingFailed(e.to_string()));
        }
    };
    let data: NoteResponse = match serde_json::from_str(&body_text) {
        Ok(d) => d,
        Err(e) => return Err(GetRemoteActivtyErrors::JsonParsingFailed(e.to_string())),
    };
    let profile =
        FullProfile::get_by_server_id_or_create(&data.attributed_to, instance.id, pool.clone())
            .await
            .map_err(|_| GetRemoteActivtyErrors::UserQueryFailed)?;
    if data.context.1.contains_key("3dModel") {
        Ok(ModelOrNote::Model(
            FullModelWithRelationsIds::create_or_get_from_note_response(
                data,
                instance_host.to_string(),
                profile.id,
                pool,
            )
            .await
            .map_err(|_| GetRemoteActivtyErrors::ModelCreationFailed)?,
        ))
    } else {
        Ok(ModelOrNote::Note(
            FullNote::create_or_get_from_note_response(data, profile.id, pool)
                .await
                .map_err(|e| {
                    error!(e = %e);
                    GetRemoteActivtyErrors::NoteCreationFailed
                })?,
        ))
    }
}
