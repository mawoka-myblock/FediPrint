use crate::helpers::auth::UserState;
use crate::helpers::sign::sign_post_request_with_body;
use crate::helpers::{Config, internal_app_error};
use crate::models::activitypub::{FollowRequest, Profile};
use crate::models::data::Webfinger;

use anyhow::Context;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::PooledConnection;
use tracing::debug;
use uuid::Uuid;
use crate::models::db::profile::{CreateProfile, ExtendedCreateProfile, FullProfile};
use crate::Pool;

pub async fn create_remote_profile(
    username: String,
    domain: String,
    pool: Pool,
) -> anyhow::Result<FullProfile> {
    let mut conn = pool.get().await?;
    let webfinger_response = reqwest::get(format!(
        "https://{domain}/.well-known/webfinger?resource=acct:{username}@{domain}"
    ))
        .await?
        .json::<Webfinger>()
        .await?;
    let mut server_id = None;
    for link in webfinger_response.links {
        if link.rel != "self" {
            continue;
        }
        server_id = Some(link.href.with_context(|| "server_id is None")?);
    }
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
    );
    let ap_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let ap_profile_response = ap_client
        .get(server_id.unwrap())
        .send()
        .await?
        .json::<Profile>()
        .await?;
    debug!("{:?}", ap_profile_response);
    use crate::schema::Profile as diesel_profile;
    Ok(diesel::insert_into(diesel_profile::table)
        .values(&ExtendedCreateProfile {
            id: Uuid::now_v7(),
            username: ap_profile_response.preferred_username.clone(),
            server: domain,
            server_id: ap_profile_response.id,
            display_name: ap_profile_response.name,
            summary: "".to_string(),
            inbox: ap_profile_response.inbox,
            outbox: ap_profile_response.outbox,
            public_key: ap_profile_response.public_key.public_key_pem,
            registered_at: chrono::DateTime::parse_from_rfc3339(
                &*ap_profile_response.published,
            )?.date_naive(),
        })
        .returning(FullProfile::as_returning())
        .get_result(&mut conn)
        .await?)
}

pub async fn follow_user(
    to_follow: FullProfile,
    claims: &UserState,
    env: &Config,
) -> anyhow::Result<()> {
    let data = FollowRequest {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/{}", env.public_url, Uuid::new_v4()),
        type_field: "Follow".to_string(),
        actor: claims.server_id.clone(),
        object: to_follow.server_id.clone().to_string(),
    };
    let json_data = serde_json::to_string(&data).unwrap();
    let data_signature = sign_post_request_with_body(
        &to_follow.inbox,
        json_data.as_ref(),
        claims.private_key.clone(),
        format!("{}#main-key", &to_follow.server_id),
    )?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
    );
    headers.insert(
        "Digest",
        reqwest::header::HeaderValue::from_str(&data_signature.1).unwrap(),
    );
    headers.insert(
        "Signature",
        reqwest::header::HeaderValue::from_str(&data_signature.1).unwrap(),
    );
    let ap_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    ap_client
        .post(to_follow.inbox.clone())
        .body(json_data)
        .send()
        .await?;
    Ok(())
}
